use utoipa::{Modify, OpenApi, ToSchema};

/// Rewrites the `operationId`s of the duplicated `/api/at/...` and `/api/eu/...`
/// mounts. Both parliaments share [`crate::server::parliament_router`], so the
/// very same handler would otherwise contribute the same `operationId` twice.
pub struct DisambiguateAtEu;

impl Modify for DisambiguateAtEu {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        for (path, item) in openapi.paths.paths.iter_mut() {
            let prefix = if path.starts_with("/api/at/") {
                "at"
            } else if path.starts_with("/api/eu/") {
                "eu"
            } else {
                continue;
            };
            for operation in [
                &mut item.get,
                &mut item.put,
                &mut item.post,
                &mut item.delete,
                &mut item.options,
                &mut item.head,
                &mut item.patch,
                &mut item.trace,
            ] {
                let Some(operation) = operation else { continue };
                if let Some(id) = &operation.operation_id {
                    operation.operation_id = Some(format!("{prefix}_{id}"));
                }
            }
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Somes API",
        description = "Data API of the Somes parliamentary information platform. Every route below the `at` and `eu` prefixes serves the Austrian and the European parliament respectively.",
        version = "1.0.0",
        contact(name = "Somes", url = "https://somes.at"),
        license(name = "AGPL-3.0"),
    ),
    tags(
        (name = "general", description = "Legislative periods and seat distributions."),
        (name = "parties", description = "Parties, party strengths and coalitions."),
        (name = "departments", description = "Federal ministries and their departments."),
        (name = "topics", description = "Eurovoc and Somes topics."),
        (name = "plenar", description = "Plenary sessions and plenary dates."),
        (name = "orientation", description = "Voting orientation questions."),
        (name = "statistics", description = "Aggregated statistics over delegates, initiatives and speeches."),
        (name = "delegates", description = "Delegates, gov officials and their activities."),
        (name = "gov_proposals", description = "Ministerial proposals and their mood barometer."),
        (name = "decrees", description = "Ministerial decrees."),
        (name = "vote_results", description = "Vote results of legislative initiatives."),
        (name = "speeches", description = "Single speeches."),
        (name = "volksbg", description = "Eintragungswochen of the Volksbegehren."),
        (name = "sitemap", description = "Paginated ids for the frontend sitemap."),
        (name = "events", description = "Somes events."),
        (name = "user", description = "Accounts, bookmarks, topics and push notifications."),
        (name = "quiz", description = "Democracy day quizzes."),
        (name = "walo", description = "Wo geht's hin? orientation questions."),
    ),
    // Schemas that are only reachable through a query parameter or through another
    // schema, and therefore not collected from a request or response body.
    components(schemas(
        somes_common_lib::Sort,
        combx::EnforcementDates,
        combx::FiscalAnalysis,
        combx::Keypoint,
        combx::ProposalComplexityScope,
        somes_meilisearch_filter::FilterOp<String>,
        somes_meilisearch_filter::FilterOp<bool>,
        somes_meilisearch_filter::FilterOp<i32>,
        somes_meilisearch_filter::FilterOp<Vec<i32>>,
        somes_meilisearch_filter::FilterOp<Vec<String>>,
    )),
)]
pub struct ApiDoc;

pub fn disambiguate(openapi: &mut utoipa::openapi::OpenApi) {
    DisambiguateAtEu.modify(openapi);
}

#[derive(ToSchema, Clone, Debug)]
pub struct PartySchema {
    pub name: String,
    pub code: String,
    pub color: String,
    pub fraction: u32,
}

#[derive(ToSchema, Clone, Debug)]
pub struct PartyStatesSchema {
    pub opposition_parties: Vec<PartySchema>,
    pub coalition_parties: Vec<PartySchema>,
}
