pub mod basic_objects;

pub fn import_bpmn_file(filename: String) -> basic_objects::BpmnProcess<'static> {
    let bpmn_process_name: String = filename.replace(".bpmn", "");
    let bpmn_process =  basic_objects::BpmnProcess::new(bpmn_process_name, filename);
    bpmn_process
}

#[cfg(tests)]
mod tests {
    use super::*;
    #[test]
    fn test_import_bpmn_file() {
        let bpmn_process = import_bpmn_file("abc.bpmn");
        assert_eq!(bpmn_process.name, "abc");
    }
}
