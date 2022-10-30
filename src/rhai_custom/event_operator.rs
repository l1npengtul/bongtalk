use rhai::{Dynamic, EvalAltResult, EvalContext, Expression};

pub fn unfiltered_event_implementation(
    context: &mut EvalContext,
    inputs: &[Expression],
) -> Result<Dynamic, Box<EvalAltResult>> {
    match context.eval_expression_tree(&inputs[0])?.into_string() {
        Ok(s) => {
            
        }
        Err(why) => {}
    }
}
