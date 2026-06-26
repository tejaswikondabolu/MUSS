mod religions;
mod heresy;
mod quiz;
mod council;
mod guide;

use axum::{
    extract::{Path, Query, State},
    response::{Html, Json},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tera::{Tera, Context};
use tower_http::services::ServeDir;

struct AppState {
    tera: Tera,
}

#[derive(Deserialize)]
struct AnalyzeParams {
    code: String,
}

#[derive(Serialize)]
struct HeresyResponse {
    success: bool,
    report: Option<serde_json::Value>,
    error: Option<String>,
}

#[derive(Deserialize)]
struct QuizResultParams {
    a0: Option<String>,
    a1: Option<String>,
    a2: Option<String>,
    a3: Option<String>,
}

#[derive(Deserialize)]
struct CouncilAskParams {
    question: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let mut tera = Tera::new("templates/**/*.html.tera").expect("Failed to load templates");
    tera.autoescape_on(vec![]);

    let state = Arc::new(AppState { tera });

    let app = Router::new()
        .route("/", get(home))
        .route("/exhibit/:id", get(exhibit))
        .route("/heresy", get(heresy_page))
        .route("/heresy/analyze", get(analyze_heresy))
        .route("/pilgrimage", get(pilgrimage))
        .route("/pilgrimage/:id", get(pilgrimage_detail))
        .route("/relics", get(relics))
        .route("/wars", get(wars))
        .route("/quiz", get(quiz_page))
        .route("/quiz/result", get(quiz_compute))
        .route("/council", get(council_page))
        .route("/council/ask", get(council_ask))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🏛️  Museum of Programming Religions open at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

fn with_guide(mut ctx: Context, page: &str) -> Context {
    let wisdom = guide::get_wisdom_for_page(page);
    ctx.insert("guide_wisdom", &wisdom);
    ctx
}

async fn home(State(state): State<Arc<AppState>>) -> Html<String> {
    let mut ctx = Context::new();
    ctx.insert("religions", &religions::all_religions());
    let ctx = with_guide(ctx, "/");
    let html = state.tera.render("index.html.tera", &ctx).unwrap();
    Html(html)
}

async fn exhibit(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Html<String> {
    let mut ctx = Context::new();
    if let Some(religion) = religions::get_religion(&id) {
        ctx.insert("religion", &religion);
        let ctx = with_guide(ctx, "/exhibit/");
        let html = state.tera.render("exhibit.html.tera", &ctx).unwrap();
        Html(html)
    } else {
        ctx.insert("religion_id", &id);
        let ctx = with_guide(ctx, "/exhibit/");
        let html = state.tera.render("404.html.tera", &ctx).unwrap();
        Html(html)
    }
}

async fn heresy_page(State(state): State<Arc<AppState>>) -> Html<String> {
    let ctx = with_guide(Context::new(), "/heresy");
    let html = state.tera.render("heresy.html.tera", &ctx).unwrap();
    Html(html)
}

async fn analyze_heresy(
    State(_state): State<Arc<AppState>>,
    Query(params): Query<AnalyzeParams>,
) -> Json<HeresyResponse> {
    if params.code.trim().is_empty() {
        return Json(HeresyResponse {
            success: false,
            report: None,
            error: Some("No code provided for analysis.".to_string()),
        });
    }

    let compilation = heresy::detector::analyze_code(&params.code);

    if compilation.success {
        let mut warnings_translated: Vec<String> = compilation
            .warnings
            .iter()
            .map(|w| heresy::translator::translate_warning(w))
            .collect();
        if warnings_translated.is_empty() {
            warnings_translated.push("The Supreme Compiler has reviewed your offering. No violations detected. The ritual is accepted.".to_string());
        }

        return Json(HeresyResponse {
            success: true,
            report: Some(serde_json::json!({
                "status": "clean",
                "religion": "Church of Rust",
                "message": "No heresy detected. Your code is pure.",
                "warnings": warnings_translated,
            })),
            error: None,
        });
    }

    let error = match compilation.errors.first() {
        Some(e) => e,
        None => {
            return Json(HeresyResponse {
                success: false,
                report: None,
                error: Some("Could not parse compiler output.".to_string()),
            })
        }
    };

    let report = heresy::translator::translate_error(error);
    let narrative = heresy::ai::generate_narrative(&error.message, error.code.as_deref());

    let warnings_translated: Vec<String> = compilation
        .warnings
        .iter()
        .map(|w| heresy::translator::translate_warning(w))
        .collect();

    Json(HeresyResponse {
        success: true,
        report: Some(serde_json::json!({
            "status": "heresy",
            "religion": report.religion,
            "violation": report.violation,
            "law_violated": report.law_violated,
            "explanation": report.explanation,
            "severity": report.severity,
            "penance": report.penance,
            "narrative_title": narrative.title,
            "narrative_story": narrative.story,
            "narrative_verdict": narrative.verdict,
            "warnings": warnings_translated,
            "error_code": error.code,
            "line": error.line,
            "column": error.column,
        })),
        error: None,
    })
}

async fn pilgrimage(State(state): State<Arc<AppState>>) -> Html<String> {
    let mut ctx = Context::new();
    ctx.insert("religions", &religions::all_religions());
    let ctx = with_guide(ctx, "/pilgrimage");
    let html = state.tera.render("pilgrimage.html.tera", &ctx).unwrap();
    Html(html)
}

async fn pilgrimage_detail(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Html<String> {
    let mut ctx = Context::new();
    if let Some(religion) = religions::get_religion(&id) {
        ctx.insert("religion", &religion);
        let ctx = with_guide(ctx, "/pilgrimage");
        let html = state.tera.render("pilgrimage_detail.html.tera", &ctx).unwrap();
        Html(html)
    } else {
        ctx.insert("religion_id", &id);
        let ctx = with_guide(ctx, "/pilgrimage");
        let html = state.tera.render("404.html.tera", &ctx).unwrap();
        Html(html)
    }
}

async fn relics(State(state): State<Arc<AppState>>) -> Html<String> {
    let mut ctx = Context::new();
    let all = religions::all_religions();
    let all_relics: Vec<_> = all
        .iter()
        .flat_map(|r| {
            r.relics.iter().map(move |rel| {
                serde_json::json!({
                    "religion_name": r.name,
                    "religion_id": r.id,
                    "name": rel.name,
                    "description": rel.description,
                    "power": rel.power,
                })
            })
        })
        .collect();
    ctx.insert("relics", &all_relics);
    let ctx = with_guide(ctx, "/relics");
    let html = state.tera.render("relics.html.tera", &ctx).unwrap();
    Html(html)
}

async fn wars(State(state): State<Arc<AppState>>) -> Html<String> {
    let wars_data = serde_json::json!([
        {
            "title": "The Great Memory War",
            "side_a": { "name": "Church of Rust", "id": "rust", "position": "Safety is sacred. Memory must be protected at all costs." },
            "side_b": { "name": "Ancient Empire of C++", "id": "cpp", "position": "Freedom is sacred. Programmers shall manage their own memory." },
            "description": "The most heated conflict in software civilization. Rust's Church believes ownership and borrowing are divine laws. C++'s Empire defends raw pointers and manual memory management as ancient rights. The war has raged since Rust's founding in 2015.",
            "casualties": "Countless segfaults, buffer overflows, and heated forum debates.",
        },
        {
            "title": "The Readability Schism",
            "side_a": { "name": "Order of Python", "id": "python", "position": "Code should read like prose. Indentation is structural." },
            "side_b": { "name": "Ancient Empire of C++", "id": "cpp", "position": "Code should be fast. Syntax is secondary to performance." },
            "description": "A theological divide over what code should look like. Python's Order uses whitespace as divine structure. C++'s Empire uses braces and semicolons. Each finds the other's scripture difficult to read.",
            "casualties": "Infinite arguments about curly braces and semicolons.",
        },
        {
            "title": "The Static vs Dynamic Inquisition",
            "side_a": { "name": "Church of Rust", "id": "rust", "position": "Types must be known at compile time. The compiler should catch all errors." },
            "side_b": { "name": "Cult of JavaScript", "id": "javascript", "position": "Types are fluid. Values can become anything at runtime." },
            "description": "A fundamental disagreement about the nature of types. Rust's Church demands compile-time type checking. The Cult of JavaScript embraces runtime flexibility. Neither can comprehend the other's worldview.",
            "casualties": "Thousands of 'undefined is not a function' and 'cannot move out of borrow' errors.",
        },
        {
            "title": "The Great Linting War",
            "side_a": { "name": "Order of Python", "id": "python", "position": "Style is doctrine. PEP 8 is the one true way." },
            "side_b": { "name": "Cult of JavaScript", "id": "javascript", "position": "Style is flexible. Prettier formats everything anyway." },
            "description": "Both religions have strong opinions on code style. Python's Order enforces strict formatting rules. The Cult of JavaScript argues over semicolons, commas, and formatting tools endlessly.",
            "casualties": "Countless tabs-versus-spaces arguments and deleted eslintrc files.",
        },
    ]);
    let mut ctx = Context::new();
    ctx.insert("wars", &wars_data);
    let ctx = with_guide(ctx, "/wars");
    let html = state.tera.render("wars.html.tera", &ctx).unwrap();
    Html(html)
}

async fn quiz_page(State(state): State<Arc<AppState>>) -> Html<String> {
    let mut ctx = Context::new();
    ctx.insert("questions", &quiz::get_questions());
    let results = quiz::all_results();
    ctx.insert("results", &results);
    let ctx = with_guide(ctx, "/quiz");
    let html = state.tera.render("quiz.html.tera", &ctx).unwrap();
    Html(html)
}

#[derive(Serialize)]
struct QuizResponse {
    result: quiz::QuizResult,
}

async fn quiz_compute(Query(params): Query<QuizResultParams>) -> Json<QuizResponse> {
    let scores = [
        params.a0.as_deref().unwrap_or("0").parse::<u32>().unwrap_or(0),
        params.a1.as_deref().unwrap_or("0").parse::<u32>().unwrap_or(0),
        params.a2.as_deref().unwrap_or("0").parse::<u32>().unwrap_or(0),
        params.a3.as_deref().unwrap_or("0").parse::<u32>().unwrap_or(0),
    ];
    let result = quiz::compute_result(scores);
    Json(QuizResponse { result })
}

async fn council_page(State(state): State<Arc<AppState>>) -> Html<String> {
    let mut ctx = Context::new();
    ctx.insert("members", &council::MEMBERS.iter().collect::<Vec<_>>());
    let ctx = with_guide(ctx, "/council");
    let html = state.tera.render("council.html.tera", &ctx).unwrap();
    Html(html)
}

async fn council_ask(Query(params): Query<CouncilAskParams>) -> Json<Vec<council::CouncilResponse>> {
    Json(council::get_debate(&params.question))
}
