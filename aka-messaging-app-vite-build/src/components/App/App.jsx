import { useEffect, useState } from "react";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import "bootstrap/dist/css/bootstrap.min.css";
import LoginPage from "../LoginPage/LoginPage";
import RegistrationPage from "../RegistrationPage/RegistrationPage";
import NotFound from "../NotFound/NotFound";
import AccessForbidden from "../AccessForbidden/AccessForbidden";
import LandingPage from "../LandingPage/LandingPage";

import './App.css';
import Navbar from "../Navbar/Navbar";


export default function AppContainer() {
  return (
      <App />
  );
}


function App() {
  const [user, setUser] = useState(false);
  return (
    <div className="App">
      <BrowserRouter>
        <Navbar user={user}
                setUser={setUser}/>
        <Routes>
          <Route path="/" element={<LandingPage                                             
                                            user={user}
                                            setUser={setUser} />} />
          <Route path="/login" element={<LoginPage                                             
                                            user={user}
                                            setUser={setUser} />} />
          <Route path="/register" element={<RegistrationPage 
                                            user={user}
                                            setUser={setUser} />} />
          <Route path="*" element={<NotFound />} />
          <Route path="/denied" element={<AccessForbidden />} />
        </Routes>
      </BrowserRouter>
    </div>
  );
}

