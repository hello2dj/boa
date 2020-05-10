initSidebarItems({"enum":[["Value","Represents any valid JSON value."]],"fn":[["from_reader","Deserialize an instance of type `T` from an IO stream of JSON."],["from_slice","Deserialize an instance of type `T` from bytes of JSON text."],["from_str","Deserialize an instance of type `T` from a string of JSON text."],["from_value","Interpret a `serde_json::Value` as an instance of type `T`."],["to_string","Serialize the given data structure as a String of JSON."],["to_string_pretty","Serialize the given data structure as a pretty-printed String of JSON."],["to_value","Convert a `T` into `serde_json::Value` which is an enum that can represent any valid JSON data."],["to_vec","Serialize the given data structure as a JSON byte vector."],["to_vec_pretty","Serialize the given data structure as a pretty-printed JSON byte vector."],["to_writer","Serialize the given data structure as JSON into the IO stream."],["to_writer_pretty","Serialize the given data structure as pretty-printed JSON into the IO stream."]],"macro":[["json","Construct a `serde_json::Value` from a JSON literal."]],"mod":[["de","Deserialize JSON data to a Rust data structure."],["error","When serializing or deserializing JSON goes wrong."],["map","A map of String to serde_json::Value."],["ser","Serialize a Rust data structure into JSON data."],["value","The Value enum, a loosely typed way of representing any valid JSON value."]],"struct":[["Deserializer","A structure that deserializes JSON into Rust values."],["Error","This type represents all possible errors that can occur when serializing or deserializing JSON data."],["Map","Represents a JSON key/value type."],["Number","Represents a JSON number, whether integer or floating point."],["Serializer","A structure for serializing Rust values into JSON."],["StreamDeserializer","Iterator that deserializes a stream into multiple JSON values."]],"type":[["Result","Alias for a `Result` with the error type `serde_json::Error`."]]});