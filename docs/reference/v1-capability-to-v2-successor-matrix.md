# V1 Capability To V2 Successor Matrix

The Assurance-era command and representation are retired, but their user capabilities have explicit Verification V2 successors. The frozen assertion-level evidence is listed below; several historical tests may point to the same current proof because they asserted the same business capability.

| V1 area | Verification V2 successor |
| --- | --- |
| Git Procedure retrieval | exact local-build or OCI package acquisition; Git transport rejected |
| AssuranceProcedure parsing | controlled VerificationProcedure/Activity/Action admission |
| start collection | `nape start` and one atomically selected current run |
| collect evidence | repeated one-file `nape evidence`, stable read, digest storage, atomic association |
| evaluate evidence | argument-free `nape verify` over frozen current-run inputs |
| AssuranceReport | VerificationReport plus Evidence Set relationship and raw evidence |
| mutable run files | complete versioned current-run state and atomic commits |
| caller metadata | bounded top-level Report metadata with reserved Engine keys |
| output discovery | NAPE-managed result location returned by Verify Receipt |

No `nape collect`, Assurance artifact, Git/HTTPS repository, one-shot Verify, caller workspace, caller output, or caller-selected run handle is a current compatibility path.
