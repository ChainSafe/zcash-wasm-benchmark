use protobuf::descriptor::field_descriptor_proto::Label;
use protobuf::descriptor::field_descriptor_proto::Type;
use protobuf::reflect::FieldDescriptor;
use protobuf::reflect::MessageDescriptor;
use protobuf_codegen::Customize;
use protobuf_codegen::CustomizeCallback;
fn main() {
    struct GenSerde;

    impl CustomizeCallback for GenSerde {
        fn message(&self, _message: &MessageDescriptor) -> Customize {
            Customize::default().before("#[wasm_bindgen::prelude::wasm_bindgen]")
        }

        fn field(&self, field: &FieldDescriptor) -> Customize {
            if field.proto().type_() == Type::TYPE_MESSAGE
                && field.proto().label() == Label::LABEL_REPEATED
            {
                Customize::default()
                    .before("#[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)]")
            } else if field.proto().type_() == Type::TYPE_ENUM
                || field.proto().type_() == Type::TYPE_MESSAGE
            {
                Customize::default().before("#[wasm_bindgen::prelude::wasm_bindgen(skip)]")
            } else if field.proto().type_() == Type::TYPE_BYTES
                && field.proto().label() == Label::LABEL_REPEATED
            {
                Customize::default().before("#[wasm_bindgen::prelude::wasm_bindgen(skip)]")
            } else if field.proto().type_() == Type::TYPE_BYTES
                || field.proto().label() == Label::LABEL_REPEATED
                || field.proto().type_() == Type::TYPE_STRING
            {
                Customize::default()
                    .before("#[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)]")
            } else {
                Customize::default()
            }
        }

        fn special_field(&self, _message: &MessageDescriptor, _field: &str) -> Customize {
            Customize::default().before("#[wasm_bindgen::prelude::wasm_bindgen(skip)]")
        }
    }

    protobuf_codegen::Codegen::new()
        .includes(&["protos"])
        .input("protos/service.proto")
        .input("protos/compact_formats.proto")
        .out_dir("src/codegen")
        .pure()
        .customize_callback(GenSerde)
        .run_from_script();
    // println!("cargo:rerun-if-changed=src/protos/service.proto");
}
