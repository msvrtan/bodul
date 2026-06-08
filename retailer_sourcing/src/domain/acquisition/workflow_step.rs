#[derive(Debug)]
pub struct WorkflowContext {
    pub retailer_id: String,
}

#[derive(Debug)]
pub struct WorkflowResult {
    pub success: bool,
    pub message: String,
}

pub trait WorkflowStep: Send + Sync {
    fn execute(&self, ctx: &WorkflowContext) -> Result<WorkflowResult, String>;
    fn name(&self) -> &str;
}
