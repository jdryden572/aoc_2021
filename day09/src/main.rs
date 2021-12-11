use std::{collections::VecDeque, time::Instant};

use plotters::{prelude::*, coord::Shift};

mod basin;
use basin::{Matrix, parse_values, BasinSearcher, Position, PositionType};

fn main() {
    let drawing_area = BitMapBackend::gif(
        "images/animated.gif", 
        (1000, 1000), 
        1_00  /* Each frame show .1s */
    ).unwrap().into_drawing_area();

    let values = parse_values("input.txt");
    let mut matrix = Matrix::new_with_low_points(values);

    let mut step = 0;
    let mut searchers = matrix.low_points().into_iter().map(|p| BasinSearcher::new(p, &matrix)).collect::<VecDeque<_>>();
    let mut finished = Vec::new();

    draw_to(&drawing_area, &mut matrix, &searchers, step);
    drawing_area.present().unwrap();

    while !searchers.is_empty() {
        let start = Instant::now();
        step += 1;
        print!("Step {}, {} basins remaining. ", step, searchers.len());
        for _ in 0..searchers.len() {
            let mut searcher = searchers.pop_front().unwrap();
            searcher.step(&mut matrix);
            if !searcher.is_done() {
                searchers.push_back(searcher);
            } else {
                finished.push(searcher);
            }
        }
        let calc_done = Instant::now();
        print!("Calc[{:?}] ", calc_done - start);
        draw_to(&drawing_area, &mut &matrix, &searchers, step);
        drawing_area.present().unwrap();
        println!("Render[{:?}]", Instant::now() - calc_done);
    }

    for _  in 0..20 {
        // Pad last frame with 2 seconds
        drawing_area.present().unwrap();
    }
}

fn draw_to(drawing_area: &DrawingArea<BitMapBackend, Shift>, matrix: &Matrix, searchers: &VecDeque<BasinSearcher>, step: i32) {
    drawing_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&drawing_area)
        .build_cartesian_2d(0.0..100.0, 0.0..100.0)
        .unwrap();

    ctx.draw_series(matrix.iter().map(rect_for)).unwrap();
    ctx.draw_series(searchers.iter().filter_map(circle_for)).unwrap();

    ctx.configure_mesh().draw().unwrap();

    let text = format!("{}", step);
    let style = TextStyle::from(("sans-serif", 20).into_font()).color(&RED);
    drawing_area.draw_text(&text, &style, (0, 0)).unwrap();
}

fn rect_for(pos: &Position) -> Rectangle<(f64, f64)> {
    //println!("{},{}", pos.x, pos.y);
    let (x, y) = (pos.x as f64, pos.y as f64);
    let color = match pos.kind {
        PositionType::LowPoint => BLUE.mix(0.5).filled(),
        PositionType::InBasin => GREEN.mix(0.5).filled(),
        PositionType::HighPoint => BLACK.mix(0.5).filled(),
        PositionType::Unknown => WHITE.mix(1.0).filled(),
    };
    Rectangle::new([(x, y), (x + 1.0, y + 1.0)], color)
}

fn circle_for(searcher: &BasinSearcher) -> Option<Circle<(f64, f64), i32>> {
    let next = searcher.frontier.get(searcher.frontier.len() - 1);
    next.map(|(x, y)| {
        let (x, y) = (*x as f64, *y as f64);
        Circle::new((x + 0.5, y + 0.5), 5, BLACK.mix(1.0).filled())
    })
}
