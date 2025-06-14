use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{
    backend::Backend, layout::{Constraint, Layout}, prelude::{Buffer, Rect}, style::{Color, Style, Stylize}, symbols::border, text::Line, widgets::{Block, Borders, Gauge, Paragraph, Widget}, DefaultTerminal, Frame
};

use material::colors;
use ratatui::prelude::*;
const SIZE: usize = 17;
const BOMB:i32 = -2;
const FOOD:i32 = -1;
const CAUDA:i32 = 1;
const VAZIO:i32 = 0;

use std::{io, str::FromStr};
fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App {
        exit: false,
        board: [0; SIZE * SIZE],
        n: 1,
        offset: 0,
        i_cobrinha: 0,
        i_comida: 0,
        i_cauda: 0,
        i_proximo: 0,
        best: 0,
        score: 0,
        level: 1,
        historico: vec![],
        mensagem: "😀 Bem Vindo ".to_string(),
    };

    app.new_game();
    let app_result = app.run(&mut terminal);

    ratatui::restore();
    app_result
}

// #[derive(Default)]
pub struct App {
    exit: bool,
    board: [i32; SIZE*SIZE],
    offset: i16,
    n: i32,
    i_cobrinha: usize,
    i_comida: usize,
    i_cauda: usize,
    i_proximo: usize,
    best: u16,
    score: u16,
    level: u16, 
    historico: Vec<Premios>,
    mensagem: String,

}

pub struct Premios {
    jogador: String,
    score: u16,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            match crossterm::event::read()? {
                crossterm::event::Event::Key(key_event) => self.handle_key_event(key_event)?,
                _ => {}
            };
            self.update();
            terminal.draw(|frame| self.draw(frame))?;

        }

        Ok(())
    }
    
    fn update(&mut self) {
        
        //Ganhou
        if self.n as usize == SIZE * SIZE   {
            self.winner();
            return
        }
        
        //Colidiu
        if colide(self.i_cobrinha, self.offset) {
            self.gameover();
            return
        }

        //Computou proximo x, y
        self.i_proximo = compute_proximo(self.i_cobrinha, self.offset).unwrap(); 

        //Inverteu
        if self.board[self.i_proximo] == self.n-1 {
            self.revert();
        } 
        self.offset = 0;
        
        //Embaracou
        if self.i_proximo > 0 &&
           self.board[self.i_proximo] > 1 {
            self.gameover();
            return
        }

        //Engordou
        if self.board[self.i_proximo] == FOOD {
            self.board[self.i_proximo] = self.n+1;
            self.n+=1;
            self.score += 1;
            self.level = self.score / 11 + 1;
            self.i_cobrinha = self.i_proximo;
            
            let mut k = seed();
            while self.board[k] != 0 {
                k = seed(); 
            }
            self.i_comida = k;
            self.board[k] = FOOD;
            return
        }

        //Apenas anda
        self.mover();
    }

    fn draw(&self, frame: &mut Frame ) {
        frame.render_widget(self, frame.area());
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent ) -> io::Result<()> {
        match (key_event.kind, key_event.code) {
            (KeyEventKind::Press, KeyCode::Char('x')) => { self.exit = true;},
            (KeyEventKind::Press, KeyCode::Char('w')) => { self.up();},
            (KeyEventKind::Press, KeyCode::Char('a')) => { self.left();},
            (KeyEventKind::Press, KeyCode::Char('s')) => { self.down();},
            (KeyEventKind::Press, KeyCode::Char('d')) => { self.right();},
            (KeyEventKind::Press, KeyCode::Char('n')) => { self.new_game();},

            (KeyEventKind::Press, KeyCode::Up) => {self.up(); }
            (KeyEventKind::Press, KeyCode::Left) => {self.left(); }
            (KeyEventKind::Press, KeyCode::Down) => {self.down(); }
            (KeyEventKind::Press, KeyCode::Right) => {self.right(); }
            _ => {}
        }
        Ok(())
    }

    fn new_game(&mut self) {
        self.board = [0; SIZE * SIZE];
        self.offset = 0;
        self.n = 1;
        self.i_cobrinha = seed();
        self.i_cauda = self.i_cobrinha;
        self.i_comida = self.i_cobrinha;
        while self.i_comida == self.i_cobrinha  {
            self.i_comida = seed();
        }

        self.board[self.i_cobrinha] = 1;
        self.board[self.i_comida] = -1;

        self.i_proximo = 0;
        self.score = 0;
        self.level = 1;
        self.mensagem = "Novo jogo gerado...".to_string();
    }

    fn gameover(&mut self) {
        // for x in 6..SIZE-6 { 
        //     for y in 6..SIZE-6 {
        //         if (x + y)%2 == 0 {
        //             self.board[index(x as u32, y as u32)] = BOMB;
        //         }
        //     }
        // }
        self.mensagem = "Fim do jogo...".to_string();
    }

    fn winner(&mut self) {
        for x in 6..SIZE-6 { 
            for y in 6..SIZE-6 {
                if (x + y)%2 == 0 {
                    self.board[index(x as u32, y as u32)] = self.n;
                }
            }
        }
        self.mensagem = "🏆 Parabéns você ganhou!".to_string();
    }
    
    fn revert(&mut self) {
        if self.n == CAUDA {
            return;
        }

        let tone = self.n + 1;
        for i in 0..SIZE*SIZE { 
                if self.board[i] == CAUDA  {
                    self.i_cobrinha = i;
                    if let Some(value) = compute_proximo(self.i_cobrinha, self.offset) {
                        self.i_proximo = value;
                    } else {
                        return
                    };
                    break;
                }
        }
        for i in 0..SIZE*SIZE { 
                if self.board[i] > VAZIO  {
                    self.board[i] = tone - self.board[i];
                }
        }
    }

    fn mover(&mut self) {
        for i in 0..SIZE*SIZE { 
            if self.board[i] >= CAUDA {
                self.board[i] = self.board[i] -1;
           }
        }
        self.board[self.i_proximo] = self.n;
        self.i_cobrinha = self.i_proximo;
    }

    fn up(&mut self) {
        self.offset = -(SIZE as i16);
    }
    
    fn down(&mut self) {
        self.offset = SIZE as i16;        
    }
    
    fn left(&mut self) {
        self.offset = -1;
        
    }
    
    fn right(&mut self) {
        self.offset = 1;
    }

    fn x(& self) -> usize  {
        self.i_cobrinha % SIZE
    }
    fn y(& self) -> usize  {
        self.i_cobrinha / SIZE

    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized {

            let layout = Layout::vertical([
                Constraint::Length(1), // Title
                Constraint::Min(0),    // Matrix
            ]);
            let [title_area, matrix_area] = layout.areas(area);
    
            // Title
            let title_line = Line::styled(
                "vs1",
                Style::default().fg(Color::from_str(colors::RED_400.to_string().as_str()).unwrap()),
            );
            Paragraph::new(title_line).render(title_area, buf);
            render_matrix(matrix_area, buf, &self.board);
            render_info_box(matrix_area, buf, self);

            render_message(matrix_area, buf, self);




        // Line::from("           /^\\/^\\\n").render(area, buf);
        // Line::from("         _|__|  O|\n").render(area, buf);
        // Line::from("\\/     /~     \\_/ \\\n").render(area, buf);
        // Line::from(" \\____|__________/  \\\n").render(area, buf);
        // Line::from("        \\_______      \\\n").render(area, buf);
        // Line::from("                `\\     \\                 \\\n").render(area, buf);
        // Line::from("                  |     |                  \\\n").render(area, buf);
        // Line::from("                 /      /                    \\\n").render(area, buf);
        // Line::from("                /     /                       \\\\\n").render(area, buf);
        // Line::from("              /      /                         \\ \\\n").render(area, buf);
        // Line::from("             /     /                            \\  \\\n").render(area, buf);
        // Line::from("           /     /             _----_            \\   \\\n").render(area, buf);
        // Line::from("          /     /           _/       ~-_         |   |\n").render(area, buf);
        // Line::from("         (      (        _ /    _--_    ~-_     _/   |\n").render(area, buf);
        // Line::from("          \\      ~-____/   __/    ~-_    ~-_-~    /\n").render(area, buf);
        // Line::from("            ~-_           _/          ~-_       _-~\n").render(area, buf);
        // Line::from("               ~--______-~                ~-___-~\n\n\n").render(area, buf);
    }
}

pub fn render_matrix(area: Rect, buf: &mut Buffer, data: &[i32]) {
    let mut lines = Vec::with_capacity(SIZE);

    for row in 0..SIZE {
        let mut spans = Vec::with_capacity(SIZE);
        for col in 0..SIZE {
            let value = data[row * SIZE + col];
            let (symbol, color) = match value {
                -2 => ("💣", Color::Red),
                // -1 => ("🎃", Color::Yellow),
                -1 => ("🍏", Color::Green),
                // 0 => ("││", Color::Black),
                0 => ("░░", Color::Black),
                // 0 => ("▒▒", Color::Black),
                // 0 => ("▓▓", Color::Black),
                // 0 => ("🔲", Color::DarkGray),
                1 => ("👾", Color::Green),
                _ => ("👾", Color::Green),
            };
            spans.push(Span::styled(symbol, Style::default().fg(color)));
        }
        lines.push(Line::from(spans));
    }
    
    let box_area = Rect {
        x: area.x,
        y: area.y,
        width: 36,
        height: 19,
    };

    Paragraph::new(lines)
    .block(
        Block::default()
            .title("snake 1.0")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Gray)))
    .render(box_area, buf);
}

pub fn render_info_box(area: Rect, buf: &mut Buffer, app: &App) {
    let right_box = Rect {
        x: area.x + 36,
        y: area.y,
        width: 36,
        height: 19,
    };

    let lines = vec![
        Line::from(vec![Span::styled("     ", Style::default())]),
        Line::from(vec![Span::styled(" x: ", Style::default()), Span::raw(app.x().to_string())]),
        Line::from(vec![Span::styled(" y: ", Style::default()), Span::raw(app.y().to_string())]),
        Line::from(vec![Span::styled(" score: ", Style::default()), Span::raw(app.score.to_string())]),
        Line::from(vec![Span::styled(" best: ", Style::default()), Span::raw(app.best.to_string())]),
        Line::from(vec![Span::styled(" level: ", Style::default()), Span::raw(app.level.to_string())]),
        Line::from(vec![Span::styled("     ", Style::default()) ]),
        Line::from(vec![Span::styled("     ", Style::default())]),
        Line::from(vec![Span::styled("         ", Style::default())]),
        Line::from(vec![Span::styled("      ^", Style::default())]),
        Line::from(vec![Span::styled("      w", Style::default())]),
        Line::from(vec![Span::styled("  ← a s d →  ", Style::default())]),
        Line::from(vec![Span::styled("   ", Style::default())]),
        Line::from(vec![Span::styled("      ", Style::default())]),
        Line::from(vec![Span::styled("  [x] sair", Style::default())]),
        Line::from(vec![Span::styled("  [n] novo jogo", Style::default())]),
    ];

    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .title("Status")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White)),
        );

    paragraph.render(right_box, buf);
}

pub fn render_message(area: Rect, buf: &mut Buffer, app: &App) {
    let msg = Line::from(vec![Span::styled(&app.mensagem, Style::default())]);
    

    let paragraph = Paragraph::new(msg)
        .block(
            Block::default()
                .title("Mensagem")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White)),
        );
    
    let msg_box = Rect {
            x: area.x,
            y: area.y + 19,
            width: 72,
            height: 10,
        };
    paragraph.render(msg_box, buf);
}

use rand::Rng;
fn seed() -> usize{
    let mut rng = rand::rng();
    rng.random_range(0..SIZE*SIZE)
}

fn colide(alfa: usize, offset: i16) -> bool {
    let index = alfa as u32;
    let x = col(index) as i16 + offset; // col()-1 ou col()+1 entre 0..SIZE
    let y = index as i16 + offset; //i-4 ou i+4 entre 0..SIZE*SIZE

    let out_of_bounds_x = x < 0 || x >= SIZE as i16;
    let out_of_bounds_y = y < 0 || y >= (SIZE * SIZE) as i16;

    if offset.abs() == 1 {
        out_of_bounds_x 
    } 
    else {
        out_of_bounds_y
    }
}

fn compute_proximo(alfa: usize, offset: i16) -> Option<usize> {
    if colide(alfa, offset){
        return None
    };    
    let result = alfa as i16 + offset;
    if result >= 0 && result < (SIZE*SIZE)  as i16 {
        Some(result as usize) 
    } else  {
        None
    }
}

fn lin(i: u32) -> u32 {
    i / SIZE as u32
}

fn col(i: u32) -> u32 {
    i %  SIZE as u32
}

fn _coord(alfa: u32) -> (u32, u32) {
     (lin(alfa), col(alfa))
}

fn index( a: u32, b: u32) -> usize {
    a as usize * SIZE + b as usize
}

// #[test]
// fn test_index() {
//     assert!(index(0,0)==0);
//     assert!(index(0,1)==1);
//     assert!(index(0,16)==16);
//     assert!(index(1,0)==17);
//     assert!(index(1,1)==18);
//     assert!(index(1,2)==19);
// }
// #[test]
// fn testcoord() {

//    assert!(coord(0) == (0, 0));
//    assert!(coord(1) == (0, 1));
//    assert!(coord(15) == (0, 15));
//    assert!(coord(16) == (0, 16));
//    assert!(coord(17) == (1, 0));
//    assert!(coord(18) == (1, 1));
//    assert!(coord(19) == (1, 2));
// }

