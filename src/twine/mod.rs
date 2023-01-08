use scraper::{ElementRef, Html, Selector};

pub struct Story {
    pub name: String,
    pub start_node: String,
    pub format: String,
    pub passages: Vec<PassageData>,
}

impl Story {
    pub fn from_html(html: &Html) -> Self {
        let story_data = html
            .select(&Selector::parse("tw-storydata").unwrap())
            .next()
            .expect("find tw-storydata");

        let story_data_element = story_data.value();

        let passage_selector = Selector::parse("tw-passagedata").unwrap();
        let passages = story_data
            .select(&passage_selector)
            .map(PassageData::from_element)
            .collect();

        Self {
            name: story_data_element
                .attr("name")
                .expect("find name")
                .to_string(),
            start_node: story_data_element
                .attr("startnode")
                .expect("find startnode")
                .to_string(),
            format: story_data_element
                .attr("format")
                .expect("find format")
                .to_string(),
            passages,
        }
    }
}

pub struct PassageData {
    pub pid: String,
    pub name: String,
    pub position: String,
    pub size: String,
    pub content: String,
}

impl PassageData {
    pub fn from_element(tag: ElementRef) -> Self {
        let element = tag.value();

        Self {
            pid: element.attr("pid").expect("find pid").to_string(),
            name: element.attr("name").expect("find name").to_string(),
            position: element.attr("position").expect("find position").to_string(),
            size: element.attr("size").expect("find size").to_string(),
            content: tag.text().collect(),
        }
    }
}
