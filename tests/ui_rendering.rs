use enoch::ui::app::App;
use enoch::ui::ui::render;
use ratatui::backend::TestBackend;
use ratatui::Terminal;
use std::fs;

fn render_at_size(width: u16, height: u16) -> String {
    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new(false);
    
    terminal.draw(|f| render(f, &mut app)).unwrap();
    
    // Get the rendered buffer
    let buffer = terminal.backend().buffer();
    let mut output = String::new();
    
    output.push_str(&format!("Terminal: {}x{}\n", width, height));
    output.push_str(&"=".repeat(width as usize));
    output.push('\n');
    
    for y in 0..height {
        for x in 0..width {
            let cell = buffer.get(x, y);
            output.push_str(cell.symbol());
        }
        output.push('\n');
    }
    
    output
}

#[test]
fn test_render_minimum_size() {
    let screenshot = render_at_size(80, 24);
    println!("{}", screenshot);
    
    // Save screenshot
    fs::create_dir_all("tests/screenshots").ok();
    fs::write("tests/screenshots/ui_80x24.txt", &screenshot).unwrap();
    
    // Verify no panics and basic elements present
    assert!(screenshot.contains("Enochian"));
    assert!(screenshot.len() > 0);
}

#[test]
fn test_render_medium_size() {
    let screenshot = render_at_size(100, 30);
    println!("{}", screenshot);
    
    fs::create_dir_all("tests/screenshots").ok();
    fs::write("tests/screenshots/ui_100x30.txt", &screenshot).unwrap();
    
    assert!(screenshot.contains("Enochian"));
    assert!(screenshot.contains("Army"));
}

#[test]
fn test_render_large_size() {
    let screenshot = render_at_size(132, 46);
    println!("{}", screenshot);
    
    fs::create_dir_all("tests/screenshots").ok();
    fs::write("tests/screenshots/ui_132x46.txt", &screenshot).unwrap();
    
    assert!(screenshot.contains("Enochian"));
    assert!(screenshot.contains("Status"));
}

#[test]
fn test_render_extra_large() {
    let screenshot = render_at_size(200, 60);
    println!("{}", screenshot);
    
    fs::create_dir_all("tests/screenshots").ok();
    fs::write("tests/screenshots/ui_200x60.txt", &screenshot).unwrap();
    
    assert!(screenshot.contains("Enochian"));
}

#[test]
fn test_board_scaling() {
    // Test that board scales appropriately at different sizes
    let sizes = vec![
        (80, 24, "1x1 squares"),
        (100, 30, "2x2 squares"),
        (132, 46, "3x3 squares"),
    ];
    
    for (width, height, expected) in sizes {
        let screenshot = render_at_size(width, height);
        println!("\n=== {} - {} ===", expected, format!("{}x{}", width, height));
        println!("{}", screenshot);
        
        // Verify board is present
        assert!(screenshot.contains("8") || screenshot.contains("7"), 
            "Board rank labels should be visible at {}x{}", width, height);
    }
}

#[test]
fn test_responsive_layout() {
    // Test that layout adapts correctly
    
    // Narrow terminal - should stack vertically
    let narrow = render_at_size(80, 30);
    fs::write("tests/screenshots/ui_narrow_80x30.txt", &narrow).unwrap();
    
    // Wide terminal - should show side panel
    let wide = render_at_size(150, 40);
    fs::write("tests/screenshots/ui_wide_150x40.txt", &wide).unwrap();
    
    println!("\n=== Narrow (80x30) ===");
    println!("{}", narrow);
    
    println!("\n=== Wide (150x40) ===");
    println!("{}", wide);
    
    // Both should have essential elements
    assert!(narrow.contains("Enochian"));
    assert!(wide.contains("Enochian"));
}

#[test]
fn test_all_standard_sizes() {
    let standard_sizes = vec![
        (80, 24),   // Minimum
        (80, 25),   // Slightly taller
        (100, 30),  // Medium
        (120, 40),  // Large
        (132, 46),  // Extra large
        (160, 50),  // Very large
        (200, 60),  // Huge
    ];
    
    for (width, height) in standard_sizes {
        let screenshot = render_at_size(width, height);
        let filename = format!("tests/screenshots/ui_{}x{}.txt", width, height);
        fs::write(&filename, &screenshot).unwrap();
        
        println!("\n=== Rendered {}x{} to {} ===", width, height, filename);
        
        // Verify no panics and basic structure
        assert!(screenshot.len() > 0, "Screenshot should not be empty");
        assert!(screenshot.contains("Enochian"), "Should contain title");
    }
}

#[test]
fn test_with_game_state() {
    use enoch::engine::types::Army;
    
    let backend = TestBackend::new(132, 46);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new(false);
    
    // Make some moves
    app.selected_army = Some(Army::Blue);
    let _ = app.try_select_square("e2");
    let _ = app.try_select_square("e4");
    
    terminal.draw(|f| render(f, &mut app)).unwrap();
    
    let buffer = terminal.backend().buffer();
    let mut output = String::new();
    
    for y in 0..46 {
        for x in 0..132 {
            output.push_str(buffer.get(x, y).symbol());
        }
        output.push('\n');
    }
    
    fs::write("tests/screenshots/ui_with_moves.txt", &output).unwrap();
    println!("{}", output);
    
    // Should show move history
    assert!(output.contains("Moves") || output.contains("Blue"));
}
