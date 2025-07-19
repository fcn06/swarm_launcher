use configuration::AgentA2aConfig;
use configuration::AgentPlannerConfig;

use a2a_agent_backbone::a2a_agent_logic::server::SimpleAgentServer;
use a2a_planner_backbone::a2a_agent_logic::planner_agent::PlannerAgent;
use a2a_planner_backbone::a2a_planner_server::planner_server::SimplePlannerAgentServer;

use clap::Parser;

use tracing::Level;
use tracing_subscriber::{
    prelude::*,
    fmt,
    layer::Layer,
    Registry, filter
};

/// Command-line arguments for the reimbursement server
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Configuration file path (TOML format)
    #[clap(long, default_value = "How are you today ?")]
    user_query: String,
    #[clap(long, default_value = "configuration/agent_a2a_config.toml")]
    config_file: String,
    #[clap(long, default_value = "warn")]
    log_level: String,
    #[clap(long, default_value = "simple_agent")]
    agent_type: String,
}

enum AgentType {
    SimpleAgent,
    SimplePlanner,
    PlannerServer
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Parse command-line arguments
    let args = Args::parse();

    /************************************************/
    /* Setting proper log level. Default is INFO    */
    /************************************************/ 
    let log_level = match args.log_level.as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::WARN,
    };

    let subscriber = Registry::default()
    .with(
        // stdout layer, to view everything in the console
        fmt::layer()
            .compact()
            .with_ansi(true)
            .with_filter(filter::LevelFilter::from_level(log_level))
    );

    tracing::subscriber::set_global_default(subscriber).unwrap();

    /************************************************/
    /* End of Setting proper log level              */
    /************************************************/ 

    /************************************************/
    /* Setting proper Agent Type             */
    /************************************************/ 


    let agent_type = match args.agent_type.as_str() {
        "simple_agent" => AgentType::SimpleAgent,
        "simple_planner" => AgentType::SimplePlanner,
        "planner_server" => AgentType::PlannerServer,
        _ => AgentType::SimpleAgent,
    };

    /************************************************/
    /* End of Setting proper Agent Type             */
    /************************************************/ 

    ///////////////////////////////////////////////////////////////////////////////////////////////

    match agent_type {
         
    /************************************************/
    /* Loading A2A Config File and launching        */
    /* A2A agent server                             */
    /************************************************/ 

    AgentType::SimpleAgent => {  

    // load a2a config file and initialize appropriateruntime
    let agent_a2a_config = AgentA2aConfig::load_agent_config(&args.config_file);
  
    // Create the modern server, and pass the runtime elements
    let server = SimpleAgentServer::new(agent_a2a_config.expect("Incrorrect A2A config file")).await?;

    println!("🌐 Starting HTTP server only...");
    server.start_http().await?;

    },

    /************************************************/
    /* A2A agent server launched                    */
    /* Responding to any A2A CLient                 */
    /************************************************/ 

    ///////////////////////////////////////////////////////////////////////////////////////////////
    
    /************************************************/
    /* Loading Planner Config File and launching    */
    /* A planner agent                              */
    /************************************************/ 
 
    AgentType::SimplePlanner => {  

    // load a2a config file and initialize appropriateruntime
    let agent_planner_config = AgentPlannerConfig::load_agent_config(&args.config_file)
        .expect("No planner configuration file");

    // Initialize the Planner Agent
    let mut planner_agent = PlannerAgent::new(agent_planner_config).await?;

    // --- Test Case 1 ---
    let result_1=planner_agent.submit_user_text(args.user_query.clone()).await;
    println!("Output:{:?}", result_1.output);

    },

    /************************************************/
    /* Planner agent launched                       */
    /************************************************/ 

    ///////////////////////////////////////////////////////////////////////////////////////////////
    
    /************************************************/
    /* Loading Planner Server Config File and launching    */
    /* A planner server agent                              */
    /************************************************/ 

    AgentType::PlannerServer => {  

    // load a2a config file and initialize appropriateruntime
    let agent_planner_config = AgentPlannerConfig::load_agent_config(&args.config_file);
    let server=SimplePlannerAgentServer::new(agent_planner_config.expect("REASON")).await?;

    println!("🌐 Starting HTTP server only...");
    server.start_http().await?;

    }
    /************************************************/
    /* Planner server agent launched                */
    /************************************************/ 

    };
    ///////////////////////////////////////////////////////////////////////////////////////////////
   



    Ok(())
}
