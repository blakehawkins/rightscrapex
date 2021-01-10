Take rightmove.co.uk URLs from stdin, emit scraped property details as json.

```bash
$ cargo run -- --floorplan --json < <(echo https://www.rightmove.co.uk/properties/100454543#/) 2>/dev/null | jq . -r
{
  "url": "https://www.rightmove.co.uk/properties/100454543#/",
  "summary": "Chilmington Green,\nAshford,\nKent,\nTN23 3DP",
  "human_identifier": "4 bedroom detached house for sale in Chilmington Green,\r\nAshford,\r\nKent,\r\nTN23 3DP, TN23",
  "price": "£625,000",
  "floorplan_url": "https://www.rightmove.co.uk/properties/100454543#/floorplan?activePlan=1",
  "location_image_url": "https://media.rightmove.co.uk/map/_generate?width=375&height=375&zoomLevel=15&latitude=51.12792&longitude=0.82873&signature=DkRafdTA0M7DxgCtvzGYYfVgIOE="
}
```
