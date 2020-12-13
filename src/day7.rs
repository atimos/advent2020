type Bags<'a> = std::collections::HashMap<&'a str, Vec<(usize, &'a str)>>;

fn get_bags(data: &str) -> Bags {
    fn get_content(content: &str) -> (usize, &str) {
        let mut content = content
            .trim()
            .trim_end_matches(&".")
            .trim_end_matches(&"s")
            .trim_end_matches(&" bag")
            .splitn(2, ' ');
        (content.next().unwrap().parse().unwrap(), content.next().unwrap())
    }

    crate::lines(data)
        .filter_map(|line| {
            if let [bag, content] =
                line.splitn(2, " bags contain ").collect::<Vec<&str>>().as_slice()
            {
                if content == &"no other bags." {
                    None
                } else {
                    let content = content.split(',');
                    let mut bags = Vec::new();

                    for content in content {
                        bags.push(get_content(content));
                    }
                    Some((*bag, bags))
                }
            } else {
                None
            }
        })
        .collect()
}

pub mod part1 {
    use super::{get_bags, Bags};
    use std::collections::HashSet;
    pub fn run() {
        let data = &crate::data("day7");
        let all_bags = get_bags(data);

        let found = find(&all_bags, "shiny gold");
        dbg!(found.len() - 1);
    }

    fn find<'a>(all_bags: &'a Bags, bag_to_find: &'a str) -> HashSet<&'a str> {
        let mut found_bags = HashSet::new();
        found_bags.insert(bag_to_find);

        for (bag, content) in all_bags {
            for (_, content_bag) in content {
                if content_bag == &bag_to_find {
                    found_bags = found_bags.union(&find(all_bags, bag)).copied().collect();
                }
            }
        }

        found_bags
    }
}

pub mod part2 {
    use super::{get_bags, Bags};
    pub fn run() {
        let data = &crate::data("day7");
        let all_bags = get_bags(data);

        dbg!(find(&all_bags, "shiny gold", 1) - 1);
    }

    fn find<'a>(all_bags: &Bags<'a>, bag_to_be_found: &'a str, bag_count: usize) -> usize {
        let mut new_count = bag_count;
        if let Some(content) = all_bags.get(bag_to_be_found) {
            for (content_count, bag) in content {
                new_count += find(all_bags, bag, bag_count * content_count);
            }
        }

        new_count
    }
}
