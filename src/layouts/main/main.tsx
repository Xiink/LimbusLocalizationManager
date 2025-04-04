import Navbar from "@/components/navbar/navbar";
import { Outlet } from "react-router";
import styles from "./main.module.css";

function Main() {
  return (
    <div className={styles.container}>
      <Navbar />
      <div className={styles.page}>
        <Outlet />
      </div>
    </div>
  );
}

export default Main;
