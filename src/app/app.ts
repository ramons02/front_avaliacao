import { Component, signal } from '@angular/core';
import { RouterOutlet, RouterLink } from '@angular/router'; // Removi o RouterLinkActive daqui

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [RouterOutlet, RouterLink], // Removi daqui também
  templateUrl: './app.html',
  styleUrl: './app.css'
})
export class App {
  protected readonly title = signal('front_avaliacao');
}
