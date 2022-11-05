(function (globalThis) {
  globalThis.fetch = function(uri) {
    console.log(Deno.core);
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

  globalThis.createContext = function() {

  }
})(globalThis);
