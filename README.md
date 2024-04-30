### Tutorial 9 Reflection Answers
1. Let us discuss this one by one. In Unary RPC, it involve a single request and a single response. So basically, this is most suitable in a simple request-response interaction between client and server where no streaming from the server is needed. Now when it is needed, server streaming RPC cacn be used because this is involving a single request with reponse being in the form of a stream form the server. This means that this is suitable for scenarios where the server needs to send multiple large amount of data in response to that request such as sending transaction history that is in a large size and needs to be divided to multiple parts. Lastly, bidirectional streaming RPC involve a bidirectional stream or the stream is now both for requests and responses between the interaction of client and server. This is most suitable for scenarios where both client and server are communicating in real-time via a chat application for example. Each of them would need to send a continuous stream of data to each other. 
2. When talking about security considerations when implementing gRPC service in Rust, there are potential security considerations particularly regarding to authentication, authorization, and data encryption. Authentication is where the clients are ensured that they are who they are before allowing to access services or to sensitive data. Authorization on the other hand is about when a client has permission to do a certain action or request. Data encryption is when the data transmitted is encrypted to prevent external attacks such as unauthorized access to the data.
3. The potential challengers or issues that may arise when handling bidirectional streaming in Rust gRPC, especially in scenarios like chat applications are but not limited to managing concurrent streams, ensuring thread safety, handling errors of terminating streams on multiple conditions, and implementing backpressure mechanisms to prevent overwhelming data sending to both client or server.
4. The advantages and disadvantages of using the tokio_stream::wrappers::ReceiverStream for streaming responses in Rust gRPC services are this provides a convenient way too convert Tokio channel to a stream. This would further simplify integration with existing Tokio-based asynchronus code. The disadvantages are unnecessary overhead and complexity could be introduced and this would lead to overhaed problems for even simpler streaming situations.
5. Rust gRPC code can be structured for maintainability and extensibility over time by the use of modular design patterns such as module system and traits for seperation concerns and promoting code reusability. Encapsulation of gRPC service logic into reusable components can also be a viable option but it has to be easily tested and maintained. 
6. There could be some other logic being implemented such as business logic where actual business processing would be required such as processing payments. This would result to performance optimization because as the volume of payment transactions and requirements grow, efficiency and scalability have to be taken into consideration more. Asynchronous processing, caching, and load balancing are techniques that could be used to hande this. More error handling and validation to the incoming payment request are also needed while management of the transactions involving external systems could be implemented but consistency and reliability have to be present. This in turn could involve database transactions or other protocols. 
7. The impact on distributed system architecture promotes standard and efficient communication protocol compared to traditional HTTP-based APIs. This will also simplify the integration between other services and platforms that support this gRPC to be used with the distributed system architecture.
8. Advantages of HTTP/2 is that there are multiplexing, header compression, server push, and binary framing layer that HTTP/2 uses to ecapsulate and transport HTTP messages efficiently. The disadvantages is that it is complex to implement, potential compatibility issues with older infrastructure, and increased resource consumption. In comparison, HTTP/1.1 with WebSocket for REST APIs offers simplicity and more compatibility but it is lacking the advanced features and performancne imprvements provided by HTTP/2. Basically WebSocket may not be as efficient and widely supported as HTTP/2 for APIs. At the end of the day, the choice is given based on the specification, requirements, and constraints of the application.
9. The contrast between them is that REST APIs follow the request-response model that means it is less efficient for real-time communication compared to gRPC bidirectional streaming capabilities. This is because gRPC allows for more efficient communication where real-time updates or continuous data streams are required.
10. The implications of Schema-based approach of gRPC compared to JSON in REST API payloads could be divided to advantages and disadvantages. The advantage is that it improve type safety, reduced payload size, and better documentation through the protobuf schemas that is not available in REST API. The disadvantages however it is less flexible compared to JSON and may require more planning or schema management beforehand.