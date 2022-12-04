// eslint-disable-next-line @typescript-eslint/no-namespace
import Loggable = Cypress.Loggable;
import Timeoutable = Cypress.Timeoutable;
import Withinable = Cypress.Withinable;
import Shadow = Cypress.Shadow;

// eslint-disable-next-line @typescript-eslint/no-namespace
declare namespace Cypress {
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  interface Chainable<Subject> {
    getByDataTest(value: string, options?: Partial<Loggable & Timeoutable & Withinable & Shadow>): Chainable<Element>;
  }
}

function getByDataTest(value: string, options?: Partial<Loggable & Timeoutable & Withinable & Shadow>) {
  return cy.get(`[data-test=${value}]`, options);
}

Cypress.Commands.add('getByDataTest', getByDataTest);
