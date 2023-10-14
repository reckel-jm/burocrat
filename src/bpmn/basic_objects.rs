pub struct BpmnToken {
    current_object: BpmnObject,
}

pub struct BpmnProcess {
    name: String,
    file_path: String,
    first_objects: Vec<BpmnObject>,
    bpmn_objects: Vec<BpmnObject>,
}

impl BpmnProcess {
    pub fn new(&self, name: String, file_path: String) -> BpmnProcess {
        BpmnProcess {
            name,
            file_path,
            first_objects: Vec::new(),
            bpmn_objects: Vec::new(),
        }
    }
}

pub enum BpmnProcessInstanceState {
    NotStarted,
    Started,
    Finished,
    Error,
}

pub struct BpmnProcessInstance<'processinstance> {
    state: BpmnProcessInstanceState,
    id: String,
    tokens: Vec<BpmnToken>,
    bpmn_process: &'processinstance BpmnProcess,
}

pub enum BpmnTaskStatus {
    BpmnTaskNotStarted,
    BpmnTaskStarted,
    BpmnTaskCompleted,
    BpmnTaskError    
}

pub struct BpmnObject<'bpmnobjectslifetime> {
    /// A unique id of the bpmn object
    pub id: String,
    /// A human readable title which can contain multiple words
    pub title: String,
    previous_objects: 'bpmnobjectslifetime+Vec<&BpmnObject>,
    next_objects: Vec<BpmnObject>,
}

impl BpmnObject {
    fn new(id: &String, title: &String) -> BpmnObject {
        BpmnObject {
            id: id.clone(),
            title: title.clone(),
            previous_objects: Vec::new(),
            next_objects: Vec::new(),
        }
    }
    pub fn set_id(&mut self, id: &String) {
        self.id = id.clone();
    }
    pub fn get_id(&self) -> &String {
        &self.id 
    }
    pub fn attach_next_object(&mut self, next_bpmn_object: &BpmnObject) {
        self.next_objects.push(&next_bpmn_object);
    }
}

pub trait BpmnConnector {
    
}

pub trait BpmnEvent {
    
}

#[cfg(test)]
mod test {
    use super::BpmnObject;

    fn create_bpmn_object() {
        let test_id: String = String::from("01");
        let bpmn_object: BpmnObject = BpmnObject::new(&test_id, &String::from("Hello"));
        let return_id: &String = bpmn_object.get_id();
        assert_eq!(&test_id, return_id);
    }
}