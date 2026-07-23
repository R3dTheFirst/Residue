use ferrisetw::provider::Provider;
use ferrisetw::parser::Parser;
use ferrisetw::schema_locator::SchemaLocator;
use ferrisetw::EventRecord;
use ferrisetw::trace::{UserTrace, TraceTrait};

pub fn start_process_monitor() {
    let process_provider = Provider::by_guid("22fb2cd6-0e7b-422b-a0c7-2fad1fd0e716")
        .any(0x10)
        .add_callback(|record: &EventRecord, schema_locator: &SchemaLocator| {
            if let Ok(schema) = schema_locator.event_schema(record) {
                let parser = Parser::create(record, &schema);
                let pid: u32 = parser.try_parse("ProcessID").unwrap_or_default();
                let image_name: String = parser
                    .try_parse("ImageName")
                    .unwrap_or_else(|_| "unknown".to_string());

                println!("[process event] PID: {pid} | Image: {image_name}");
            }
        })
        .build();
    
    let (mut trace, _handle) = UserTrace::new()
        .named("ResidueProcessTrace".to_string())
        .enable(process_provider)
        .start()
        .expect("Failed to start ETW trace");

    trace.process().expect("Trace processing failed");
}