import * as path from "path";
import * as fs from "fs";
import { DeployEnvironment } from "./types";

export class OldEnvironmentRepository {
  private _environment: DeployEnvironment[] = [];
  private readonly _environmentPath: string;
  private readonly _osmiumPath: string;

  constructor(workspacePath: string) {
    this._osmiumPath = path.join(workspacePath, ".osmium");
    this._environmentPath = path.join(this._osmiumPath, "environments.json");
    this.load();
  }

  public load(): void {
    if (!fs.existsSync(this._osmiumPath)) {
      fs.mkdirSync(this._osmiumPath);
    }
    if (fs.existsSync(this._environmentPath)) {
      const environmentData = fs.readFileSync(this._environmentPath, "utf8");
      const environmentJson = JSON.parse(environmentData);
      this._environment = environmentJson.environments;
    } else {
      fs.writeFileSync(
        this._environmentPath,
        JSON.stringify({ environments: [] }),
      );
      this._environment = [];
    }
  }

  public getEnvironments(): DeployEnvironment[] {
    return this._environment;
  }

  public getEnvironment(name: `0x${string}`): DeployEnvironment | undefined {
    return this._environment.find((w) => w.name === name);
  }

  public async createEnvironment(
    environment: DeployEnvironment,
  ): Promise<DeployEnvironment> {
    if (this._environment.find((w) => w.name === environment.name)) {
      this._environment = this._environment.map((w) => {
        if (w.name === environment.name) {
          return environment;
        }
        return w;
      });
    } else {
      this._environment.push(environment);
    }

    const environmentJson = JSON.stringify({ environments: this._environment });
    fs.writeFileSync(this._environmentPath, environmentJson, "utf8");
    return environment;
  }

  public async deleteEnvironment(name: string): Promise<void> {
    this._environment = this._environment.filter((w) => w.name !== name);
    const environmentJson = JSON.stringify({ environments: this._environment });
    fs.writeFileSync(this._environmentPath, environmentJson, "utf8");
  }
}
