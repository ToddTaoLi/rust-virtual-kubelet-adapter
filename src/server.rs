use iron::error::HttpError;
use iron::Listening;
use iron::prelude::*;
use persistent::State;
use router::Router;

use provider::Provider;
use routes;
use utils;
use types::ProviderState;

pub fn start_server<T>(prov: Box<T>) -> Result<Listening, HttpError>
where
    T: Provider + 'static + Send + Sync,
{
    // create a chain with a route map
    let mut chain = Chain::new(setup_route_map::<T>());

    let provider_state = ProviderState { prov };

    // this object manages thread-safe access to the shared provider state
    let safe_provider_state = State::<ProviderState<T>>::one(provider_state);

    // add a "before" middleware for injecting our provider state
    chain.link_before(safe_provider_state);

    // start the web server
    let port = utils::get_env_integral("PORT", Ok(3000u16));
    Iron::new(chain).http(format!("0.0.0.0:{}", port))
}

fn setup_route_map<T>() -> Router
where
    T: Provider + 'static + Send + Sync,
{
    router!(
        index: get "/" => routes::default,
        create_pod: post "/createPod" => routes::create_pod::<T>,
        update_pod: put "/updatePod" => routes::update_pod::<T>,
        delete_pod: delete "/deletePod" => routes::delete_pod::<T>,
        get_pod: get "/getPod" => routes::get_pod::<T>,
        get_container_logs: get "/getContainerLogs" => routes::get_container_logs::<T>,
        get_pod_status: get "/getPodStatus" => routes::get_pod_status::<T>,
        get_pods: get "/getPods" => routes::get_pods::<T>,
        capacity: get "/capacity" => routes::capacity::<T>,
        node_conditions: get "/nodeConditions" => routes::node_conditions::<T>,
        node_addresses: get "/nodeAddresses" => routes::node_addresses::<T>,
        node_daemon_endpoints: get "/nodeDaemonEndpoints" => routes::node_daemon_endpoints::<T>,
        operating_system: get "/operatingSystem" => routes::operating_system::<T>,
    )
}
