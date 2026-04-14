import { Component, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { PacienteService, Paciente } from '../../pacientes/paciente.service';
import jsPDF from 'jspdf';
import html2canvas from 'html2canvas';

@Component({
  selector: 'app-avaliacao-form',
  standalone: true,
  imports: [CommonModule, FormsModule],
  templateUrl: './avaliacao-form.html',
  styleUrl: './avaliacao-form.css'
})
export class AvaliacaoFormComponent implements OnInit {
  listaPacientes: Paciente[] = [];
  pacienteSelecionado?: Paciente;

  avaliacao = {
    pacienteId: '',
    singleHopDireita: 0, singleHopEsquerda: 0,
    tripleHopDireita: 0, tripleHopEsquerda: 0,
    crossoverHopDireita: 0, crossoverHopEsquerda: 0,
    sixMeterDireita: 0, sixMeterEsquerda: 0
  };

  dataAtual = new Date();

  constructor(private pacienteService: PacienteService) {}

  ngOnInit(): void {
    this.pacienteService.listarTodos().subscribe({
      next: (dados) => {
        this.listaPacientes = dados;
      },
      error: (err) => console.error('Erro ao buscar pacientes:', err)
    });
  }

  onPacienteChange() {
    this.pacienteSelecionado = this.listaPacientes.find(p => p.id === this.avaliacao.pacienteId);
  }

  get lsi() {
    return {
      single: this.calcularLSI(this.avaliacao.singleHopDireita, this.avaliacao.singleHopEsquerda),
      triple: this.calcularLSI(this.avaliacao.tripleHopDireita, this.avaliacao.tripleHopEsquerda),
      crossover: this.calcularLSI(this.avaliacao.crossoverHopDireita, this.avaliacao.crossoverHopEsquerda),
      sixMeter: this.calcularLSI(this.avaliacao.sixMeterDireita, this.avaliacao.sixMeterEsquerda)
    };
  }

  calcularLSI(dir: number, esq: number): number {
    if (dir === 0 || esq === 0) return 0;
    return (Math.min(dir, esq) / Math.max(dir, esq)) * 100;
  }

  get mediaGeralLSI(): number {
    const valores = [this.lsi.single, this.lsi.triple, this.lsi.crossover, this.lsi.sixMeter];
    const soma = valores.reduce((a, b) => a + b, 0);
    return soma > 0 ? soma / 4 : 0;
  }

  get estaApto(): boolean {
    return this.mediaGeralLSI >= 90;
  }

  calcularTudo() {}

  gerarRelatorioPDF() {
    const data = document.getElementById('relatorio-pdf');
    if (!data || !this.pacienteSelecionado) {
      alert('Selecione um paciente e preencha os dados.');
      return;
    }

    html2canvas(data, { scale: 2 }).then(canvas => {
      const imgWidth = 208;
      const imgHeight = (canvas.height * imgWidth) / canvas.width;
      const contentDataURL = canvas.toDataURL('image/png');
      const pdf = new jsPDF('p', 'mm', 'a4');
      pdf.addImage(contentDataURL, 'PNG', 0, 0, imgWidth, imgHeight);
      pdf.save(`relatorio_${this.pacienteSelecionado?.nome.replace(/ /g, '_')}.pdf`);
    });
  }
}
