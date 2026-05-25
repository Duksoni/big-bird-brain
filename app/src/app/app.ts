import { Component, inject } from "@angular/core";
import { CommonModule } from "@angular/common";
import { RouterOutlet } from "@angular/router";
import { MatIconRegistry } from "@angular/material/icon";
import { DomSanitizer } from "@angular/platform-browser";

@Component({
	selector: "app-root",
	standalone: true,
	imports: [CommonModule, RouterOutlet],
	templateUrl: "./app.html",
	styleUrl: "./app.css",
})
export class App {
	private matIconRegistry = inject(MatIconRegistry);
	private domSanitizer = inject(DomSanitizer);

	constructor() {
		this.matIconRegistry.addSvgIcon(
			"exit_to_app",
			this.domSanitizer.bypassSecurityTrustResourceUrl(
				"assets/icons/exit_to_app.svg",
			),			
		)
		this.matIconRegistry.addSvgIcon(
			"refresh",
			this.domSanitizer.bypassSecurityTrustResourceUrl(
				"assets/icons/refresh.svg",
			),
		);
	}
}
