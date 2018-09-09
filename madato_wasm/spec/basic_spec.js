export let xray = import("madato_wasm");

describe("A suite", function() {
    it("contains spec with an expectation", function(done) {


      var res = madato.test_str("foo");
      expect(res == "foo").toBe(true);

      /*

      madato.then(m => {
        var md = m.test_str("foo");
        expect(res == "foo").toBe(true);
        done();
      });

      */
      
    });
});