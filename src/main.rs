use duckdb::{params, Connection};

#[derive(Debug)]
struct News {
    title: String,
}

fn main() {
    let path = "./db/us_financal_news.db";
    let conn: Connection = Connection::open(path).unwrap();

    let mut stmt = conn.prepare("SELECT title FROM demo;").unwrap();

    let news_iter = stmt
        .query_map([], |row| {
            Ok(News {
                title: row.get(0).unwrap(),
            })
        })
        .unwrap();

    let analyzer = vader_sentiment::SentimentIntensityAnalyzer::new();

    for new in news_iter {
        let title = &new.unwrap().title;
        let sentiment: String;
        let senti: std::collections::HashMap<&str, f64> = analyzer.polarity_scores(title);
        match senti.get("compound") {
            Some(s) => {
                if s >= &0.05 {
                    println!();
                    sentiment = "positive".to_owned();
                } else if s < &0.05 {
                    sentiment = "negative".to_owned();
                } else {
                    sentiment = "neutral".to_owned();
                }
            }
            None => todo!(),
        }
        conn.execute(
            "INSERT INTO sentiment (title, sentiment) VALUES (?, ?)",
            params![title, sentiment],
        )
        .unwrap();
    }

    let _ = conn.close();
}
