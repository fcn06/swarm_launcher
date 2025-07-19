#!/bin/bash

# This script launches different components of the Swarm project.
# Sample configuration files for agent, mcp capability and server are in configuration repository
# They can and need to be adjusted according to your need
# 
# Before running, make sure to set the necessary LLM API keys as environment variables.
# For example:
# export LLM_A2A_API_KEY="your_a2a_api_key_here" , corresponding to the openai compatible chat completion api that you want to use for simple planner
# export LLM_MCP_API_KEY="your_mcp_api_key_here" , corresponding to the openai compatible chat completion api that you want to use for MCP capability
# export LLM_PLANNER_API_KEY="your_planner_api_key_here" , corresponding to the openai compatible chat completion api that you want to use for Planner capability
#
# Then run 
# cargo build --release


echo "Select an agent to launch:"
echo "1) A2A Agent (requires LLM_A2A_API_KEY, optionally LLM_MCP_API_KEY)"
echo "2) Simple Planner (requires LLM_PLANNER_API_KEY)"
echo "3) Planner Server (requires LLM_PLANNER_API_KEY)"
echo "Enter your choice (1, 2, or 3):"

read choice

case $choice in
    1)
        read -p "Enter the path to the A2A Agent configuration file (default: configuration/agent_a2a_config.toml): " A2A_CONFIG_FILE
        A2A_CONFIG_FILE=${A2A_CONFIG_FILE:-"configuration/agent_a2a_config.toml"}
        echo "Launching A2A Agent..."
        ./target/release/swarm_launcher --config-file "$A2A_CONFIG_FILE" --agent-type "simple_agent" &
        ;;
    2)
        read -p "Enter the path to the Simple Planner configuration file (default: configuration/agent_planner_config.toml): " SIMPLE_PLANNER_CONFIG_FILE
        SIMPLE_PLANNER_CONFIG_FILE=${SIMPLE_PLANNER_CONFIG_FILE:-"configuration/agent_planner_config.toml"}
        
        read -p "Enter the user query for Simple Planner (default: How are you doing ?): " USER_QUERY
        USER_QUERY=${USER_QUERY:-"How are you doing ?"}

        echo "Launching Simple Planner..."
        ./target/release/swarm_launcher --config-file "$SIMPLE_PLANNER_CONFIG_FILE" --agent-type "simple_planner" --user-query "$USER_QUERY"
        ;;
    3)
        read -p "Enter the path to the Planner Server configuration file (default: configuration/agent_planner_config.toml): " PLANNER_SERVER_CONFIG_FILE
        PLANNER_SERVER_CONFIG_FILE=${PLANNER_SERVER_CONFIG_FILE:-"configuration/agent_planner_config.toml"}
        echo "Launching Planner Server..."
        ./target/release/swarm_launcher --config-file "$PLANNER_SERVER_CONFIG_FILE" --agent-type "planner_server" &
        ;;
    *)
        echo "Invalid choice. Please enter 1, 2, or 3."
        ;;
esac
