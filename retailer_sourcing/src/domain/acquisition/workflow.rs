use super::workflow_step::WorkflowStep;

pub struct Workflow {
    pub name: String,
    pub steps: Vec<Box<dyn WorkflowStep>>,
}

impl Workflow {
    pub fn new(name: String, steps: Vec<Box<dyn WorkflowStep>>) -> Self {
        Self { name, steps }
    }
}
