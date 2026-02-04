import { render } from "preact";
import { Redirect, Route, Switch } from "wouter-preact";
import { FeedbackPage } from "./pages/feedback";
import { LandingPage } from "./pages/landing";

const appRoot = document.getElementById("appRoot")!;

render(
  <Switch>
    <Route path="/feedback">
      <FeedbackPage />
    </Route>
    <Route path="/">
      <LandingPage />
    </Route>
    <Route>
      <Redirect to="/" />
    </Route>
  </Switch>,
  appRoot,
);
