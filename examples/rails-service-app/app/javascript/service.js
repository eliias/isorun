import {parseHTML} from 'linkedom';
import * as d3 from "d3";
import convert from 'color-convert';

export function generate(canvas, settings, data) {
  const {document} = parseHTML(`
    <!doctype html>
    <html lang="en">
      <body>
      </body>
    </html>
  `);

  const body = d3.select(document).select("body")

  const svg = body
    .append("svg")
    .attr("width", canvas.width)
    .attr("height", canvas.height);

  const {aggregates} = data.me;

  const {baseColor: backgroundColor, highlightColor: strokeColor} = settings;
  const strokeColorRgb = convert.hex.rgb(strokeColor.substr(1)).join();

  const cols = 7;
  const {margin} = canvas;
  const w = canvas.width - margin.left - margin.right;
  const h = canvas.width - margin.top - margin.bottom;
  const gridSize = Math.floor(w / cols);

  svg
    .append('rect')
    .attr('class', 'bg')
    .attr('style', `fill: ${backgroundColor}`)
    .attr('width', canvas.width)
    .attr('height', canvas.height);

  const groupDots = svg
    .append('g')
    .attr('class', 'groupDots')
    .attr('width', w)
    .attr('height', h)
    .attr('transform', 'translate(20, 20)');

  const dots = d => {
    const result = d.map((entry, index) => ({
      x: Math.floor(index % 7) + 1,
      y: Math.floor(index / 7) + 1,
      week: entry.interval,
      duration: entry.intervalValue
    }));

    const sizeScale = d3
      .scalePow()
      .exponent(0.7)
      .domain([0, d3.max(result, d => d.duration)])
      .range([8, gridSize / 2.2]);

    const colorOpacity = d3
      .scaleLinear()
      .domain([0, d3.max(result, d => d.duration)])
      .range([0.3, 1]);

    const dots = groupDots.selectAll('.dot').data(result, d => d.y + ':' + d.x);

    dots
      .enter()
      .append('circle')
      .attr('class', 'dot')
      .attr(
        'style',
        d => `fill: rgba(${strokeColorRgb}, ${colorOpacity(d.duration)})`
      )
      .attr('cx', d => (d.x - 1) * gridSize + gridSize / 2)
      .attr('cy', d => (d.y - 1) * gridSize + gridSize / 2)
      .attr('r', d => sizeScale(d.duration));

    dots.exit().remove();

    return body.html();
  }

  return dots(aggregates.elapsedTimeSeries);
}
