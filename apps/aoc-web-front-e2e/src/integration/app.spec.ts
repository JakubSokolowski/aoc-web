describe('aoc-web-front', () => {
  beforeEach(() => cy.visit('/'));

  it('should display container with game of life', () => {
    cy.getByDataTest('description').contains("Rust AOC solutions, run in browser with WASM:");
  });
});
