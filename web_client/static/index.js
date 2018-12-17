const rust = import("../build/web_client");

import "./index.css";

rust.then(m => m.run());
