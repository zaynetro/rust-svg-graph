# Rust SVG Graphs

Project is in the very early stage.


## TODO

* [x] Bar graph
    * [x] X-axis (line, ticks and labels)
    * [x] Y-axis (line, ticks and values)
    * [x] Justify bars to the bottom
    * [x] Add spacing between bars
    * [x] Draw grid
* [x] Line chart
* [x] Pie chart
    * [x] Labels
* [ ] Examples
    * [ ] Amount of stock sold per month
    * [ ] Page views per day for line chart
    * [ ] Operating systems for pie chart
* [x] Fill empty value separators (when the value is not present in the data)
* [x] Reuse code for plotting X,Y axes
* [ ] Write tests for common calculations
* [ ] Separate graph stylings into separate module
* [x] Use scale as an interface
* [ ] Verify edge cases work (zero entries, one entry, a lot of entries)
* [x] Handle negative values (requires rewriting Axis)
* [ ] Improve codebase (remove hacks)
* [ ] Default colour palette
* [ ] Write docs
* [ ] Combine LinearScale and LinearRoundedScale


## Notes

Svg can have inline styles:

```
<?xml version="1.0" standalone="no"?>
<svg width="200" height="200" xmlns="http://www.w3.org/2000/svg" version="1.1">
  <defs>
    <style type="text/css"><![CDATA[
       #MyRect {
         stroke: black;
         fill: red;
       }
    ]]></style>
  </defs>
  <rect x="10" height="180" y="10" width="180" id="MyRect"/>
</svg>
```
