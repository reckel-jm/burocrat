pub struct BpmnToken<'a> {
    current_object: &'a BpmnObject<'a>,
}

pub struct BpmnProcess<'a> {
    name: String,
    file_path: String,
    first_objects: Vec<&'a BpmnObject<'a>>,
    bpmn_objects: Vec<&'a BpmnObject<'a>>,
}

impl<'a> BpmnProcess<'a> {
    pub fn new (name: String, file_path: String) -> BpmnProcess<'a> {
        BpmnProcess {
            name,
            file_path,
            first_objects: Vec::new(),
            bpmn_objects: Vec::new(),
        }
    }
    
    pub fn get_name(&self) -> &String {
        &self.name
    }
}

pub enum BpmnProcessInstanceState {
    NotStarted,
    Started,
    Finished,
    Error,
}

pub struct BpmnProcessInstance<'a> {
    state: BpmnProcessInstanceState,
    id: String,
    tokens: Vec<BpmnToken<'a>>,
    bpmn_process: &'a BpmnProcess<'a>,
}

pub enum BpmnTaskStatus {
    BpmnTaskNotStarted,
    BpmnTaskStarted,
    BpmnTaskCompleted,
    BpmnTaskError    
}

pub struct BpmnObject<'a> {
    /// A unique id of the bpmn object
    pub id: String,
    /// A human readable title which can contain multiple words
    pub title: String,
    previous_objects: Vec<&'a BpmnObject<'a>>,
    next_objects: Vec<&'a BpmnObject<'a>>,
}

impl<'a> BpmnObject<'a> {
    fn new(id: &String, title: &String) -> BpmnObject<'a> {
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
    pub fn attach_next_object(&mut self, next_bpmn_object: &'a BpmnObject) {
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