use axum::routing::{post, delete};
use axum::{Json, Router};
use axum::extract::{State, Path};

use crate::model::{ModelController, Ticket, TicketForCreate};
use crate::Result;

// Route region
pub fn routes(mc: ModelController) -> Router {
    Router::new()
    .route("/tickets", post(create_ticket).get(list_tickets))
    .route("/tickets/:id", delete(delete_ticket))
    .with_state(mc)
}

async fn create_ticket(
    State(mc): State<ModelController>,
    Json(ticket_fc): Json<TicketForCreate>
) -> Result<Json<Ticket>> {
    println!(">> {:<12} - create_ticket", "Handaer");
    let ticket = mc.create_ticket(ticket_fc).await?;
    
    Ok(Json(ticket))
}

async fn list_tickets(
    State(mc): State<ModelController>
) -> Result<Json<Vec<Ticket>>> {
    println!(">> {:<12} - list_tickets", "HANDALER");

    let tickets = mc.list_tickets().await?;

    Ok(Json(tickets))
}

async fn delete_ticket(
    State(mc): State<ModelController>,
    Path(id): Path<u64>
) -> Result<Json<Ticket>> {
    println!(">> {:<12} - delte_ticket", "HANDALER");
    let ticket = mc.delete_ticket(id).await?;

    Ok(Json(ticket))
}