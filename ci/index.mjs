import fs from "node:fs";
import path from "node:path";
import url from "node:url";
import { connect } from "@dagger.io/dagger";

const EXCLUDE = { exclude: ["ci/"] };

const DIRNAME = url.fileURLToPath(new URL(".", import.meta.url));

connect(
	async (client) => {
		const hostDir = client.host().directory(".");

		const database = client
			.container()
			.from("postgres:15-alpine")
			.withEnvVariable("POSTGRES_USER", "root")
			.withEnvVariable("POSTGRES_PASSWORD", "root")
			.withExposedPort(5432)
			.withDirectory("/app", hostDir, EXCLUDE)
			.withNewFile("/docker-entrypoint-initdb.d/init.sh", {
				contents: fs.readFileSync(path.resolve(DIRNAME, "init", "pg_init.sh"), { encoding: "utf-8" }),
			});

		const server = client
			.container()
			.from("rust:1.70.0")
			.withServiceBinding("db", database)
			.withDirectory("/app", hostDir, EXCLUDE)
			.withWorkdir("/app")
			.withEnvVariable("DATABASE_HOST", "db")
			.withEnvVariable("DATABASE_TEMPLATE", process.env.DATABASE_TEMPLATE);

		await server.withExec(["cargo", "test"]).stderr();
	},
	{ LogOutput: process.stdout }
);
