// eslint-disable-next-line @typescript-eslint/no-namespace
declare namespace Cypress {
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  interface Chainable<Subject> {
    getByDataTest(value: string): Chainable<Element>;
  }
}

function getByDataTest(value: string) {
  return cy.get(`[data-test=${value}]`);
}

Cypress.Commands.add('getByDataTest', getByDataTest);
