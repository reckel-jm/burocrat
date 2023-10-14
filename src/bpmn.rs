pub mod basic_objects;
use basic_objects::*;

pub fn import_bpmn_file() -> basic_objects::BpmnProcess {
    let bpmn_process = basic_objects::BpmnProcess::new();
    bpmn_process
}