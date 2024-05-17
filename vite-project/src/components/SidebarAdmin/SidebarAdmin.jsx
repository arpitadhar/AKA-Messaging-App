import React from 'react'
import './SidebarAdmin.css'
import logo from './logo.png'
import home from './home.png'
const SidebarAdmin = () => {
    return (
        <div className='sidebarad'> 
          {/*menu*/}
          <div className = "menu">
            <div className="menuItem">
                    <img src={logo} alt = "Home"/>
                <dashboard>Dashboard</dashboard>
            </div>
            <div className="menuItem">
                    <img src={home} alt = "Home"/>
                <dashboard>home</dashboard>
            </div>
          </div>
        </div>
    )
}

export default SidebarAdmin;
