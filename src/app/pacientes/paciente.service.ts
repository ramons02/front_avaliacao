import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';

// Interface do Paciente (Dados Pessoais)
export interface Paciente {
  id?: string;
  nome: string;
  peso: string;
  altura: string;
  diasPosRLCA: string;
}

// Interface da Avaliação (Dados dos Testes e LSI)
// Atualizado para incluir o Crossover Hop conforme o modelo [cite: 8]
export interface Avaliacao {
  id?: string;
  pacienteId: string;
  dataAvaliacao: string;

  // Testes de Salto (cm)
  singleHopDir: number;
  singleHopEsq: number;
  tripleHopDir: number;
  tripleHopEsq: number;
  crossoverDir: number; // Campo novo para o relatório [cite: 8, 13]
  crossoverEsq: number; // Campo novo para o relatório [cite: 8, 13]

  // Teste de Tempo (seg)
  sixMeterDir: number;
  sixMeterEsq: number;
}

@Injectable({
  providedIn: 'root'
})
export class PacienteService {
  // SUBSTITUA pelo seu link da Railway (exemplo abaixo)
  private readonly BASE_URL = 'https://hupsteste-production.up.railway.app';

// No seu arquivo paciente.service.ts
  private readonly API_PACIENTES = 'https://hupstesteback-production.up.railway.app/api/pacientes';
  private readonly API_AVALIACOES = 'https://hupstesteback-production.up.railway.app/api/avaliacoes';

  constructor(private http: HttpClient) {}
  // ... resto do código

  // --- Métodos de Pacientes ---

  listarTodos(): Observable<Paciente[]> {
    return this.http.get<Paciente[]>(this.API_PACIENTES);
  }

  salvar(paciente: Paciente): Observable<Paciente> {
    return this.http.post<Paciente>(this.API_PACIENTES, paciente);
  }

  // --- Métodos de Avaliações (Para o Relatório HupsTeste) ---

  salvarAvaliacao(avaliacao: Avaliacao): Observable<Avaliacao> {
    return this.http.post<Avaliacao>(this.API_AVALIACOES, avaliacao);
  }

  buscarAvaliacoesPorPaciente(pacienteId: string): Observable<Avaliacao[]> {
    return this.http.get<Avaliacao[]>(`${this.API_AVALIACOES}/paciente/${pacienteId}`);
  }
}
