//! Owned MIXEL company and Print project links in the title bar, Help, About and Home.

use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use pdfcraft_engine::links;
use pdfcraft_ui_egui::{Dialog, PdfCraftApp};

fn harness(setup: impl FnOnce(&mut PdfCraftApp) + 'static) -> Harness<'static, PdfCraftApp> {
    let mut h = Harness::builder().with_size(egui::vec2(1400.0, 900.0)).build_eframe(move |_cc| {
        let mut app = PdfCraftApp::new();
        app.set_option("language", "en").unwrap();
        setup(&mut app);
        app
    });
    h.run_steps(4);
    h
}

#[test]
fn website_button_in_the_top_bar_opens_mixel() {
    let mut h = harness(|_| {});
    h.query_all_by_label("MIXEL Website").next().unwrap().click();
    h.run_steps(2);
    assert_eq!(h.state().last_opened_url.as_deref(), Some(links::WEBSITE));
    assert_eq!(links::WEBSITE, "https://www.mixel.ch");
}

#[test]
fn home_screen_links() {
    for (label, url) in [
        ("MIXEL Print web page", "https://github.com/jangopeopleV2/mixel-print"),
        ("MIXEL Print on GitHub", "https://github.com/jangopeopleV2/mixel-print"),
        ("MIXEL Website", "https://www.mixel.ch"),
    ] {
        let mut h = harness(|_| {});
        h.get_by_label("MIXEL Creative Apps");
        // Home follows the topbar in the accessibility tree; both website controls share a URL.
        h.query_all_by_label(label).last().unwrap().click();
        h.run_steps(2);
        assert_eq!(h.state().last_opened_url.as_deref(), Some(url), "{label}");
    }
}

#[test]
fn about_dialog_shows_the_brand_and_links() {
    let pdf = b"%PDF-1.7\n1 0 obj << /Type /Catalog /Pages 2 0 R >> endobj\n2 0 obj << /Type /Pages /Kids [3 0 R] /Count 1 >> endobj\n3 0 obj << /Type /Page /Parent 2 0 R /MediaBox [0 0 200 200] >> endobj\ntrailer << /Root 1 0 R >>\n%%EOF";
    // With a document open, so the home screen's own links are not on screen.
    let mut h = harness(move |app| {
        app.open_bytes("one.pdf", None, pdf.to_vec()).unwrap();
        app.dialog = Some(Dialog::About);
    });
    h.get_by_label("MIXEL");
    assert_eq!(h.query_all_by_label("Discord").count(), 0, "No unowned community control is shown");
    // The titlebar is drawn before the modal and remains in the accessibility tree.
    // Require both controls, then exercise the About link that follows the titlebar.
    let websites: Vec<_> = h.query_all_by_label("MIXEL Website").collect();
    assert_eq!(websites.len(), 2, "The titlebar and About each expose their company link");
    websites.last().unwrap().click();
    h.run_steps(2);
    assert_eq!(h.state().last_opened_url.as_deref(), Some(links::WEBSITE));
}

#[test]
fn help_commands_open_each_link() {
    for l in links::LINKS {
        let mut h = harness(|_| {});
        assert!(h.state_mut().execute(l.command), "{}", l.command);
        assert_eq!(h.state().last_opened_url.as_deref(), Some(l.url));
        assert_eq!(pdfcraft_engine::commands::command(l.command).unwrap().menu, Some("Help"));
    }
}

/// Runs frames until the node labelled `label` exists and has stopped moving, then returns it.
///
/// The About dialog is a modal centred on its own size, and switching tabs changes that size, so
/// for a few frames afterwards the modal grows and recentres and every control in it moves. A
/// fixed number of frames is a guess at how long that takes: it was too few after the 0.5.0
/// contributors refresh (78 names instead of 5) on some machines, the click on Table landed where
/// the button had been, and the table never opened (#671).
fn settled<'a>(h: &'a mut Harness<'static, PdfCraftApp>, label: &'a str) -> egui_kittest::Node<'a> {
    let mut last = None;
    for _ in 0..60 {
        h.run_steps(1);
        let now = h.query_by_label(label).map(|n| n.rect());
        if now.is_some() && now == last {
            return h.get_by_label(label);
        }
        last = now;
    }
    panic!("{label:?} never appeared or never stopped moving");
}

#[test]
fn about_dialog_has_contributors_and_models_tabs() {
    let mut h = harness(|app| app.dialog = Some(Dialog::About));
    settled(&mut h, "Contributors").click();
    // The owner is always in the compiled-in credits (contributors/contributors.json), shown by username.
    settled(&mut h, "@echelon");
    settled(&mut h, "Table").click();
    settled(&mut h, "PRs");
    settled(&mut h, "Display name").click();
    settled(&mut h, "Brandon Thomas");
    settled(&mut h, "Models").click();
    h.run_steps(4);
    assert!(h.query_all_by_label("Anthropic").count() >= 1);
}
