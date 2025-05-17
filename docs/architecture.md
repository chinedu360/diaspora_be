```mermaid
graph TD
    %% Layer: Application
    subgraph "Application Layer"
        main["main.rs<br/>Entry point & config loader"]
    end

    %% Layer: Interface
    subgraph "Interface Layer"
        startup["startup.rs<br/>HTTP server setup"]
    end

    %% Layer: API
    subgraph "API Layer"
        routes["routes/mod.rs<br/>Route exports"]
        health_check["health_check.rs<br/>Re-exported: Health check handler"]
        subscriptions["subscriptions.rs<br/>Re-exported: Subscription handlers"]
    end

    %% Layer: Domain
    subgraph "Domain Layer"
        domain["domain/<br/>Business logic"]
    end

    %% Layer: Infrastructure
    subgraph "Infrastructure Layer"
        config["configuration.rs<br/>App config"]
        db["db.rs<br/>Database connections"]
    end

    %% Import relationships
    main -->|imports| startup
    main -->|loads config| config
    startup -->|imports| routes
    routes --> health_check
    routes --> subscriptions
    health_check -.->|may use| domain
    subscriptions -.->|may use| domain
    health_check -.->|may use| db
    subscriptions -->|uses| db

    %% Data flow
    Client((Client)) -->|HTTP Request| main
    main -->|forwards to| startup
    startup -->|routes to| health_check
    startup -->|routes to| subscriptions
    health_check -->|HTTP Response| Client
    subscriptions -->|stores data in| db
    subscriptions -->|HTTP Response| Client
    config -.->|configures| db
    config -.->|configures| startup

    %% Styles
    classDef application fill:#f96,stroke:#333,stroke-width:2px;
    classDef interface fill:#9cf,stroke:#333,stroke-width:2px;
    classDef api fill:#9f9,stroke:#333,stroke-width:2px;
    classDef domain fill:#c9f,stroke:#333,stroke-width:2px;
    classDef infrastructure fill:#ff9,stroke:#333,stroke-width:2px;
    classDef external fill:#eee,stroke:#333,stroke-width:1px;

    class main application;
    class startup interface;
    class routes,health_check,subscriptions api;
    class domain domain;
    class config,db infrastructure;
    class Client external;
