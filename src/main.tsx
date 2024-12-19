import React from "react";
import ReactDOM from "react-dom/client";

import View from "./View";
import OBS from "./OBS";
import "./index.css";
import { BrowserRouter, Route, Routes, Link} from "react-router-dom";


ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    {/* <App /> */}
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<OBS/>}/>
        <Route path="/view" element={<View/>}/>
      </Routes>
    </BrowserRouter>
  </React.StrictMode>,
);
