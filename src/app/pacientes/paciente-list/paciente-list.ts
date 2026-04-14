import { Component, OnInit, ChangeDetectorRef } from '@angular/core'; // Adicionado ChangeDetectorRef
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { RouterLink } from '@angular/router';
import { PacienteService, Paciente } from '../paciente.service';

@Component({
  selector: 'app-paciente-list',
  standalone: true,
  imports: [CommonModule, FormsModule, RouterLink],
  templateUrl: './paciente-list.html',
  styleUrl: './paciente-list.css'
})
export class PacienteListComponent implements OnInit {
  pacientes: Paciente[] = [];

  novoPaciente: Paciente = {
    nome: '',
    peso: '',
    altura: '',
    diasPosRLCA: ''
  };

  // Injetamos o ChangeDetectorRef para forçar a atualização da tela se necessário
  constructor(
    private service: PacienteService,
    private cdr: ChangeDetectorRef
  ) {}

  ngOnInit(): void {
    this.carregarPacientes();
  }

  carregarPacientes(): void {
    this.service.listarTodos().subscribe({
      next: (dados: Paciente[]) => {
        this.pacientes = [...dados]; // Criamos uma nova referência de array para forçar o Angular a ver a mudança
        this.cdr.detectChanges(); // Força a atualização da tabela na tela
      },
      error: (err: any) => {
        console.error('Erro ao buscar pacientes:', err);
      }
    });
  }

  cadastrar(): void {
    // Validação simples antes de enviar
    if (this.novoPaciente.nome && this.novoPaciente.peso && this.novoPaciente.altura) {
      this.service.salvar(this.novoPaciente).subscribe({
        next: (res: Paciente) => {
          console.log('Paciente salvo:', res);
          this.carregarPacientes(); // Recarrega a lista do banco
          this.limparFormulario();
        },
        error: (err: any) => {
          console.error('Erro ao salvar:', err);
          alert('Erro ao salvar no banco de dados. Verifique a conexão com o back-end.');
        }
      });
    } else {
      alert('Por favor, preencha todos os campos obrigatórios.');
    }
  }

  private limparFormulario(): void {
    this.novoPaciente = {
      nome: '',
      peso: '',
      altura: '',
      diasPosRLCA: ''
    };
  }
}
