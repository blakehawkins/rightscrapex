Read rightmove property details pages as URLs from stdin, emit scraped data as json.

```bash
$ cargo run -- --floorplan --json < <(echo https://www.rightmove.co.uk/property-to-rent/property-68183922.html)
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/rightscrapex --floorplan --json`
{"url":"https://www.rightmove.co.uk/property-to-rent/property-68183922.html","summary":"2 bedroom flat to
rent","human_identifier":"White Hart Lane, Barnes, SW13","price":"£1,500
pcm","floorplan_url":"https://www.rightmove.co.uk/property-to-rent/property-68183922.html#floorplan","location_image_url":"//media.rightmove.co.uk/map/_generate?latitude=51.466628&longitude=-0.253700&zoomLevel=14&width=190&height=222&signature=ha1gEOpgThpWZ9oYfvRNCWCkmPY="}
```
