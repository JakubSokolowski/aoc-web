describe('nx-rs-front', () => {
  beforeEach(() => cy.visit('/'));

  it('should display container with game of life', () => {
    cy.getByDataTest('problem').contains("Year 2021 day 1 solution:");
  });
});
