use crate::router::DetailsView;
use uuid::Uuid;

pub fn create_route(
    company_id: Option<Uuid>,
    role_id: Option<Uuid>,
    interview_id: Option<Uuid>,
    view: Option<DetailsView>,
) -> String {
    let mut route = String::new();
    if let Some(company_id) = company_id {
        route.push('/');
        route.push_str(&company_id.to_string());

        if let Some(role_id) = role_id {
            route.push('/');
            route.push_str(&role_id.to_string());

            if let Some(interview_id) = interview_id {
                route.push('/');
                route.push_str(&interview_id.to_string());
            }
        }
    }

    if let Some(view) = view {
        route.push_str(&format!("?view={view}"))
    }

    route
}
