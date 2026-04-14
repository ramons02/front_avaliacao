import { Routes } from '@angular/router';
// Note que removi o ".component" do final do nome do arquivo
import { PacienteListComponent } from './pacientes/paciente-list/paciente-list';
import { AvaliacaoFormComponent } from './avaliacoes/avaliacao-form/avaliacao-form';

export const routes: Routes = [
  { path: 'pacientes', component: PacienteListComponent },
  { path: 'avaliacao', component: AvaliacaoFormComponent },
  // Redireciona o caminho vazio para pacientes
  { path: '', redirectTo: 'pacientes', pathMatch: 'full' },
  // Rota curinga: se digitar qualquer coisa errada, volta para pacientes
  { path: '**', redirectTo: 'pacientes' }
];
