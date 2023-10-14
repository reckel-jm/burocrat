pub mod basic_objects;

/// This function imports a bpmn_file and returns the BPMN process object.
pub fn import_bpmn_file(filename: String) -> basic_objects::BpmnProcess<'static> {
    let bpmn_process_name: String = filename.replace(".bpmn", "");
    let bpmn_process =  basic_objects::BpmnProcess::new(bpmn_process_name, filename);
    bpmn_process
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_import_bpmn_file() {
        let bpmn_process = import_bpmn_file(String::from("abc.bpmn"));
        assert_eq!(String::from(bpmn_process.get_name()), String::from("abc"));
    }
}
