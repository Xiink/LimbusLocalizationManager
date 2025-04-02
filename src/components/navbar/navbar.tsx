import { NavLink } from "react-router";
import styles from "./navbar.module.css";
import { cn } from "@/utils";
import { Home, Info, Settings, Map } from "lucide-react";

function Navbar() {
  return (
    <nav className={styles.container}>
      <NavLink 
        to="/"
        className={getLinkClassName}
      >
        <Home className="w-8 h-8" />
      </NavLink>
      <NavLink to="/localizations" className={getLinkClassName}>
        <Map className="w-8 h-8" />
      </NavLink>
      <NavLink to="/settings" className={getLinkClassName}>
        <Settings className="w-8 h-8" />
      </NavLink>
      <NavLink to="/about" className={getLinkClassName}>
        <Info className="w-8 h-8" />
      </NavLink>
    </nav>
  );

  function getLinkClassName({ isActive, isPending }: { isActive: boolean, isPending: boolean }) {
    return cn(
      styles.link, 
      isPending ? styles.pending : isActive ? styles.active : ""
    );
  }
}

export default Navbar;
