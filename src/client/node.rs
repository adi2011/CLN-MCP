use cln_grpc::pb::node_client::NodeClient;
use cln_grpc::pb::{
    BkprchannelsapyRequest, BkprlistaccounteventsRequest, BkprlistbalancesRequest,
    BkprlistincomeRequest, CheckmessageRequest, DecodeRequest, DecodepayRequest, FeeratesRequest,
    GetinfoRequest, GetlogRequest, GetrouteRequest, ListaddressesRequest, ListchannelsRequest,
    ListclosedchannelsRequest, ListconfigsRequest, ListdatastoreRequest, ListforwardsRequest,
    ListfundsRequest, ListhtlcsRequest, ListinvoicerequestsRequest, ListinvoicesRequest,
    ListnodesRequest, ListoffersRequest, ListpaysRequest, ListpeerchannelsRequest,
    ListpeersRequest, ListsendpaysRequest,
};
use rmcp::{model::*, tool, Error as McpError};
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{transport::Channel, Request, Response};

macro_rules! doc_from_file {
    ($path:expr) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/docs/", $path))
    };
}

#[derive(Clone)]
pub struct NodeService {
    client: Arc<Mutex<NodeClient<Channel>>>,
}

fn get_response<T>(res: Result<Response<T>, tonic::Status>) -> Result<CallToolResult, McpError>
where
    T: Serialize,
{
    match res {
        Ok(response) => {
            // Convert the response into a JSON-serializable format
            let response_data = serde_json::to_value(response.into_inner())
                .map_err(|_| McpError::internal_error("Failed to serialize response", None))?;

            Ok(CallToolResult::success(vec![
                Content::json(response_data).unwrap()
            ]))
        }
        Err(e) => Err(McpError::internal_error(
            format!("Failed to communicate with lightning node: {}", e),
            None,
        )),
    }
}

#[tool(tool_box)]
impl NodeService {
    pub fn new(channel: Channel) -> Self {
        Self {
            client: Arc::new(Mutex::new(NodeClient::new(channel))),
        }
    }

    // Node Information
    #[tool(description = doc_from_file!("getinfo.md"))]
    pub async fn get_info(&self) -> Result<CallToolResult, McpError> {
        let request = Request::new(GetinfoRequest::default());
        let mut client = self.client.lock().await;

        let res = client.getinfo(request).await;
        get_response(res)
    }

    #[tool(description = doc_from_file!("listconfigs.md"))]
    pub async fn list_configs(&self) -> Result<CallToolResult, McpError> {
        let request = Request::new(ListconfigsRequest::default());
        let mut client = self.client.lock().await;

        let res = client.list_configs(request).await;
        get_response(res)
    }

    #[tool(description = doc_from_file!("listaddresses.md"))]
    pub async fn list_addresses(&self) -> Result<CallToolResult, McpError> {
        let request = Request::new(ListaddressesRequest::default());
        let mut client = self.client.lock().await;

        let res = client.list_addresses(request).await;
        get_response(res)
    }

    // Channel Information
    #[tool(description = doc_from_file!("listchannels.md"))]
    pub async fn list_channels(&self) -> Result<CallToolResult, McpError> {
        let request = Request::new(ListchannelsRequest::default());
        let mut client = self.client.lock().await;

        let res = client.list_channels(request).await;
        get_response(res)
    }

    #[tool(description = doc_from_file!("listpeerchannels.md"))]
    pub async fn list_peer_channels(&self) -> Result<CallToolResult, McpError> {
        let request = Request::new(ListpeerchannelsRequest::default());
        let mut client = self.client.lock().await;

        let res = client.list_peer_channels(request).await;
        get_response(res)
    }

    #[tool(description = doc_from_file!("listclosedchannels.md"))]
    pub async fn list_closed_channels(&self) -> Result<CallToolResult, McpError> {
        let request = Request::new(ListclosedchannelsRequest::default());
        let mut client = self.client.lock().await;

        let res = client.list_closed_channels(request).await;
        get_response(res)
    }

    #[tool(description = doc_from_file!("listhtlcs.md"))]
    pub async fn list_htlcs(&self) -> Result<CallToolResult, McpError> {
        let request = Request::new(ListhtlcsRequest::default());
        let mut client = self.client.lock().await;

        let res = client.list_htlcs(request).await;
        get_response(res)
    }

    // Payment Information
    #[tool(description = doc_from_file!("listpays.md"))]
    pub async fn list_pays(&self) -> Result<CallToolResult, McpError> {
        let request = Request::new(ListpaysRequest::default());
        let mut client = self.client.lock().await;

        let res = client.list_pays(request).await;
        get_response(res)
    }

    #[tool(description = doc_from_file!("listsendpays.md"))]
    pub async fn list_send_pays(&self) -> Result<CallToolResult, McpError> {
        let request = Request::new(ListsendpaysRequest::default());
        let mut client = self.client.lock().await;

        let res = client.list_send_pays(request).await;
        get_response(res)
    }

    #[tool(description = doc_from_file!("listforwards.md"))]
    pub async fn list_forwards(&self) -> Result<CallToolResult, McpError> {
        let request = Request::new(ListforwardsRequest::default());
        let mut client = self.client.lock().await;

        let res = client.list_forwards(request).await;
        get_response(res)
    }

    #[tool(description = doc_from_file!("listinvoices.md"))]
    pub async fn list_invoices(&self) -> Result<CallToolResult, McpError> {
        let request = Request::new(ListinvoicesRequest::default());
        let mut client = self.client.lock().await;

        let res = client.list_invoices(request).await;
        get_response(res)
    }

    #[tool(description = doc_from_file!("listinvoicerequests.md"))]
    pub async fn list_invoice_requests(&self) -> Result<CallToolResult, McpError> {
        let request = Request::new(ListinvoicerequestsRequest::default());
        let mut client = self.client.lock().await;

        let res = client.list_invoice_requests(request).await;
        get_response(res)
    }

    // Network Information
    #[tool(description = doc_from_file!("listpeers.md"))]
    pub async fn list_peers(&self) -> Result<CallToolResult, McpError> {
        let request = Request::new(ListpeersRequest::default());
        let mut client = self.client.lock().await;

        let res = client.list_peers(request).await;
        get_response(res)
    }

    #[tool(description = doc_from_file!("listnodes.md"))]
    pub async fn list_nodes(&self) -> Result<CallToolResult, McpError> {
        let request = Request::new(ListnodesRequest::default());
        let mut client = self.client.lock().await;

        let res = client.list_nodes(request).await;
        get_response(res)
    }

    #[tool(description = doc_from_file!("listfunds.md"))]
    pub async fn list_funds(&self) -> Result<CallToolResult, McpError> {
        let request = Request::new(ListfundsRequest::default());
        let mut client = self.client.lock().await;

        let res = client.list_funds(request).await;
        get_response(res)
    }

    #[tool(description = doc_from_file!("getroute.md"))]
    pub async fn get_route(&self) -> Result<CallToolResult, McpError> {
        let request = Request::new(GetrouteRequest::default());
        let mut client = self.client.lock().await;

        let res = client.get_route(request).await;
        get_response(res)
    }

    // Offer Information
    #[tool(description = doc_from_file!("listoffers.md"))]
    pub async fn list_offers(&self) -> Result<CallToolResult, McpError> {
        let request = Request::new(ListoffersRequest::default());
        let mut client = self.client.lock().await;

        let res = client.list_offers(request).await;
        get_response(res)
    }

    // Database Information
    #[tool(description = doc_from_file!("listdatastore.md"))]
    pub async fn list_datastore(&self) -> Result<CallToolResult, McpError> {
        let request = Request::new(ListdatastoreRequest::default());
        let mut client = self.client.lock().await;

        let res = client.list_datastore(request).await;
        get_response(res)
    }

    // Bookkeeping Information
    #[tool(description = doc_from_file!("bkpr-channelsapy.md"))]
    pub async fn bkpr_channels_apy(&self) -> Result<CallToolResult, McpError> {
        let request = Request::new(BkprchannelsapyRequest::default());
        let mut client = self.client.lock().await;

        let res = client.bkpr_channels_apy(request).await;
        get_response(res)
    }

    #[tool(description = doc_from_file!("bkpr-listbalances.md"))]
    pub async fn bkpr_list_balances(&self) -> Result<CallToolResult, McpError> {
        let request = Request::new(BkprlistbalancesRequest::default());
        let mut client = self.client.lock().await;

        let res = client.bkpr_list_balances(request).await;
        get_response(res)
    }

    #[tool(description = doc_from_file!("bkpr-listincome.md"))]
    pub async fn bkpr_list_income(&self) -> Result<CallToolResult, McpError> {
        let request = Request::new(BkprlistincomeRequest::default());
        let mut client = self.client.lock().await;

        let res = client.bkpr_list_income(request).await;
        get_response(res)
    }

    #[tool(description = doc_from_file!("bkpr-listaccountevents.md"))]
    pub async fn bkpr_list_account_events(&self) -> Result<CallToolResult, McpError> {
        let request = Request::new(BkprlistaccounteventsRequest::default());
        let mut client = self.client.lock().await;

        let res = client.bkpr_list_account_events(request).await;
        get_response(res)
    }

    // Utility Commands
    #[tool(description = doc_from_file!("decode.md"))]
    pub async fn decode(&self) -> Result<CallToolResult, McpError> {
        let request = Request::new(DecodeRequest::default());
        let mut client = self.client.lock().await;

        let res = client.decode(request).await;
        get_response(res)
    }

    #[tool(description = doc_from_file!("decodepay.md"))]
    pub async fn decode_pay(&self) -> Result<CallToolResult, McpError> {
        let request = Request::new(DecodepayRequest::default());
        let mut client = self.client.lock().await;

        let res = client.decode_pay(request).await;
        get_response(res)
    }

    #[tool(description = doc_from_file!("checkmessage.md"))]
    pub async fn check_message(&self) -> Result<CallToolResult, McpError> {
        let request = Request::new(CheckmessageRequest::default());
        let mut client = self.client.lock().await;

        let res = client.check_message(request).await;
        get_response(res)
    }

    #[tool(description = doc_from_file!("feerates.md"))]
    pub async fn feerates(&self) -> Result<CallToolResult, McpError> {
        let request = Request::new(FeeratesRequest::default());
        let mut client = self.client.lock().await;

        let res = client.feerates(request).await;
        get_response(res)
    }

    #[tool(description = doc_from_file!("getlog.md"))]
    pub async fn get_log(&self) -> Result<CallToolResult, McpError> {
        let request = Request::new(GetlogRequest::default());
        let mut client = self.client.lock().await;

        let res = client.get_log(request).await;
        get_response(res)
    }
}

#[tool(tool_box)]
impl rmcp::ServerHandler for NodeService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            instructions: Some("Core Lightning Node".into()),
            server_info: Implementation::from_build_env(),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
        }
    }
}
