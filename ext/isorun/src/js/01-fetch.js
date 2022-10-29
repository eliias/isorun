(function (window) {
  window.fetch = function(uri) {
    // intercept call and return result
    const content = JSON.stringify(
      {
        "data": {
          "testField": "Hello World!"
        }
      }
    );

    const response = new Response(content);
    return Promise.resolve(response);
  }
})(globalThis);
