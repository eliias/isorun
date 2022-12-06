class PagesController < ApplicationController
  def index
    service = Isorun::Module.load("service")
    generate = service.import("default")
    @result = generate.call(canvas, settings, data)
  end

  private

  def canvas
    width = 874
    height = 1240

    {
      width: width,
      height: height,
      margin: {
        top: width / 12,
        right: width / 16,
        bottom: width / 18,
        left: width / 16
      }
    }
  end

  def settings
    {
      baseColor: "#221D23",
      highlightColor: "#D1603D",
      activitiesEndDate: "2017-12-31T20:17:38+00:00"
    }
  end

  def data
    {
      "me": {
        "units": "METRIC",
        "aggregates": {
          "summary": {
            "count": 103,
            "types": %w[HIKE RIDE RUN]
          },
          "elapsedTime": {
            "min": 1855,
            "avg": 6203.065217391304,
            "max": 16212,
            "total": 285341
          },
          "elapsedTimeSeries": [
            {
              "interval": 1,
              "intervalValue": 0,
              "year": 2017
            },
            {
              "interval": 2,
              "intervalValue": 5577,
              "year": 2017
            },
            {
              "interval": 3,
              "intervalValue": 0,
              "year": 2017
            },
            {
              "interval": 4,
              "intervalValue": 0,
              "year": 2017
            },
            {
              "interval": 5,
              "intervalValue": 0,
              "year": 2017
            },
            {
              "interval": 6,
              "intervalValue": 0,
              "year": 2017
            },
            {
              "interval": 7,
              "intervalValue": 0,
              "year": 2017
            },
            {
              "interval": 8,
              "intervalValue": 0,
              "year": 2017
            },
            {
              "interval": 9,
              "intervalValue": 0,
              "year": 2017
            },
            {
              "interval": 10,
              "intervalValue": 0,
              "year": 2017
            },
            {
              "interval": 11,
              "intervalValue": 0,
              "year": 2017
            },
            {
              "interval": 12,
              "intervalValue": 8940,
              "year": 2017
            },
            {
              "interval": 13,
              "intervalValue": 8460,
              "year": 2017
            },
            {
              "interval": 14,
              "intervalValue": 2340,
              "year": 2017
            },
            {
              "interval": 15,
              "intervalValue": 15301,
              "year": 2017
            },
            {
              "interval": 16,
              "intervalValue": 4200,
              "year": 2017
            },
            {
              "interval": 17,
              "intervalValue": 3060,
              "year": 2017
            },
            {
              "interval": 18,
              "intervalValue": 3004,
              "year": 2017
            },
            {
              "interval": 19,
              "intervalValue": 5411,
              "year": 2017
            },
            {
              "interval": 20,
              "intervalValue": 11809,
              "year": 2017
            },
            {
              "interval": 21,
              "intervalValue": 11846,
              "year": 2017
            },
            {
              "interval": 22,
              "intervalValue": 9489,
              "year": 2017
            },
            {
              "interval": 23,
              "intervalValue": 7734,
              "year": 2017
            },
            {
              "interval": 24,
              "intervalValue": 21453,
              "year": 2017
            },
            {
              "interval": 25,
              "intervalValue": 1949,
              "year": 2017
            },
            {
              "interval": 26,
              "intervalValue": 4707,
              "year": 2017
            },
            {
              "interval": 27,
              "intervalValue": 14112,
              "year": 2017
            },
            {
              "interval": 28,
              "intervalValue": 11910,
              "year": 2017
            },
            {
              "interval": 29,
              "intervalValue": 32127,
              "year": 2017
            },
            {
              "interval": 30,
              "intervalValue": 27199,
              "year": 2017
            },
            {
              "interval": 31,
              "intervalValue": 38420,
              "year": 2017
            },
            {
              "interval": 32,
              "intervalValue": 22347,
              "year": 2017
            },
            {
              "interval": 33,
              "intervalValue": 0,
              "year": 2017
            },
            {
              "interval": 34,
              "intervalValue": 31406,
              "year": 2017
            },
            {
              "interval": 35,
              "intervalValue": 15176,
              "year": 2017
            },
            {
              "interval": 36,
              "intervalValue": 18724,
              "year": 2017
            },
            {
              "interval": 37,
              "intervalValue": 9287,
              "year": 2017
            },
            {
              "interval": 38,
              "intervalValue": 8061,
              "year": 2017
            },
            {
              "interval": 39,
              "intervalValue": 11049,
              "year": 2017
            },
            {
              "interval": 40,
              "intervalValue": 11414,
              "year": 2017
            },
            {
              "interval": 41,
              "intervalValue": 7739,
              "year": 2017
            },
            {
              "interval": 42,
              "intervalValue": 13757,
              "year": 2017
            },
            {
              "interval": 43,
              "intervalValue": 14655,
              "year": 2017
            },
            {
              "interval": 44,
              "intervalValue": 2096,
              "year": 2017
            },
            {
              "interval": 45,
              "intervalValue": 2216,
              "year": 2017
            },
            {
              "interval": 46,
              "intervalValue": 0,
              "year": 2017
            },
            {
              "interval": 47,
              "intervalValue": 0,
              "year": 2017
            },
            {
              "interval": 48,
              "intervalValue": 2286,
              "year": 2017
            },
            {
              "interval": 49,
              "intervalValue": 0,
              "year": 2017
            },
            {
              "interval": 50,
              "intervalValue": 4995,
              "year": 2017
            },
            {
              "interval": 51,
              "intervalValue": 0,
              "year": 2017
            },
            {
              "interval": 52,
              "intervalValue": 0,
              "year": 2017
            },
            {
              "interval": 53,
              "intervalValue": 0,
              "year": 2017
            }
          ]
        }
      }
    }
  end
end
