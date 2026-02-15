fn function() {
    // Dummy to satisfy syntax checker for step_executor.rs which uses ::SchedulerContext
}

pub trait SchedulerContext {
    fn send_message(&mut self, target_id: usize, msg: Value);
}
// wait, I need to make sure `step_executor` uses the traits correctly.
// Let's redefine `step_executor.rs` to use `SchedulerContext` defined here or in lib.
