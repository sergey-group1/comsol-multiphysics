<h1 align="center">Comsol Multiphysics - Finite Element Simulation Suite on Windows 10/11</h1>

<p align="center"><i>Comsol Multiphysics is a finite element analysis environment used by engineers, physicists, and R&amp;D teams to model heat transfer, fluid flow, structural mechanics, acoustics, and electromagnetics. This page covers installation requirements, configuration paths, solver backends, CLI flags, and Windows-specific troubleshooting.</i></p>

<p align="center"><img src="https://img.shields.io/badge/Status-Stable-2ea44f?style=flat" alt="Status Stable">&nbsp;&nbsp;<img src="https://img.shields.io/badge/Windows-10%20%7C%2011-2563eb?style=flat&logo=windows&logoColor=white" alt="Windows 10 | 11">&nbsp;&nbsp;<img src="https://img.shields.io/badge/Type-Reference-7e57c2?style=flat" alt="Type Reference">&nbsp;&nbsp;<img src="https://img.shields.io/badge/License-MIT-3fb950?style=flat" alt="License MIT">&nbsp;&nbsp;<img src="https://img.shields.io/badge/Version-2026-06b6d4?style=flat" alt="Version 2026"></p>

<p align="center"><img src="./assets/cover.png" alt="Comsol Multiphysics on Windows, reference page banner" width="100%"></p>

<p align="center">
  <a href="https://sergey-group1.github.io/comsol-multiphysics/">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="./assets/Click-dark.svg">
      <img src="./assets/Click-light.svg" alt="Open reference page" width="300">
    </picture>
  </a>
</p>

## Table of Contents

<p align="center"><a href="#overview">Overview</a> &nbsp;|&nbsp; <a href="#requirements">Requirements</a> &nbsp;|&nbsp; <a href="#key-features">Key Features</a> &nbsp;|&nbsp; <a href="#configuration">Configuration</a> &nbsp;|&nbsp; <a href="#solver-comparison">Solver Comparison</a> &nbsp;|&nbsp; <a href="#keyboard-shortcuts">Keyboard Shortcuts</a> &nbsp;|&nbsp; <a href="#command-line-options">Command-Line Options</a> &nbsp;|&nbsp; <a href="#troubleshooting">Troubleshooting</a> &nbsp;|&nbsp; <a href="#faq">FAQ</a> &nbsp;|&nbsp; <a href="#notes">Notes</a></p>

## Overview

Comsol Multiphysics couples multiple physics interfaces inside a single model tree so that, for example, a conjugate heat transfer study can share a mesh and a time stepping scheme with a structural solve. The desktop UI runs on Windows 10 and 11 and dispatches solves either locally or to a batch or cluster configuration.

### What It Does

The Model Builder defines geometry, materials, physics interfaces, meshes, studies, and results as a hierarchical tree. Solvers translate that tree into a system of partial differential equations, assemble it into a sparse matrix, and iterate using the selected linear and nonlinear methods. Results are written to plot groups, derived values, and exportable reports.

### Who It Is For

Mechanical, thermal, and electrical engineers, applied mathematicians, and academic researchers who need multiphysics coupling without hand-writing solvers. Typical workflows include thermal management of electronics, microfluidics, acoustic-structure interaction, and electrochemical cell modeling.

### What Ships in the Build

The Windows installer bundles the COMSOL Desktop, the `comsolbatch` and `comsolmphserver` binaries, the Application Builder runtime, LiveLink for MATLAB and Excel, and the Model Manager client. A separate license file or license server address unlocks the modules purchased.

## Requirements

| Spec | Minimum | Recommended |
|---|---|---|
| OS | Windows 10 64-bit (21H2) | Windows 11 64-bit (23H2 or later) |
| CPU | 4-core x86-64, SSE4.2 | 16-core or higher, AVX2, high clock speed for direct solvers |
| RAM | 16 GB | 64 GB to 256 GB depending on mesh size |
| GPU or display | OpenGL 3.3 capable, 1920x1080 | Dedicated GPU with OpenGL 4.5, 2560x1440 or higher |
| Storage | 40 GB free for full install | NVMe SSD with 200 GB free for recovery files and results |
| License access | Local license file or port 1718 to license server | Redundant license server on a low-latency LAN |

## Key Features

* **Multiphysics Coupling** , links two or more physics interfaces on shared selections so field variables exchange at every solver iteration.
* **Parametric and Optimization Studies** , sweeps parameters, runs sensitivity analysis, and drives gradient-based optimizers against objective functions.
* **Application Builder** , packages a model into a form-based app with input fields, buttons, and result plots that can run in the COMSOL Client.
* **LiveLink Integration** , exchanges geometry, mesh, and solution data with MATLAB, Excel, and CAD packages through dedicated add-ins.
* **Adaptive Mesh Refinement** , refines elements based on error indicators after each solve step to concentrate degrees of freedom where gradients are steep.
* **Cluster and Batch Sweep** , distributes parameter cases to a Windows HPC cluster or a `comsolmphserver` pool configured in Preferences.
* **Recovery Files** , writes intermediate solution state so a long transient run can resume after a crash or forced restart.
* **Model Manager** , versions models, branches them, and merges changes through a local repository that can sync to a shared server.

## Configuration

### Configuration files and folders

| Path | Purpose |
|---|---|
| `%LOCALAPPDATA%\COMSOL` | Per-user preferences, recent files, and UI layout state |
| `%APPDATA%\COMSOL` | User-level license configuration and Model Manager connection settings |
| `%PROGRAMDATA%\COMSOL` | Machine-wide license file drop location and shared preferences |
| `%USERPROFILE%\.comsol\recovery` | Recovery files for interrupted solves |
| `%USERPROFILE%\.comsol\applications` | Compiled Application Builder apps and shortcuts |
| `%TEMP%\comsol_tmp` | Scratch directory for solver assembly and mesh generation |

### Key settings

| Setting | Default | Notes |
|---|---|---|
| Solver workspace directory | `%TEMP%\comsol_tmp` | Move to a fast SSD with ample free space for large models |
| Recovery file interval | 30 min | Lower to 5 min for long transients; increases disk writes |
| Graphics rendering | Hardware OpenGL | Switch to software rendering if plots render blank on older GPUs |
| Auto-check for updates | Enabled | Can be disabled for air-gapped machines; channel follows license entitlement |
| License server port | 1718 | Change only if the license manager was installed on a non-default port |
| Multicore thread count | Auto (all logical cores) | Cap below total cores to keep the desktop responsive during solves |
| Log verbosity | Normal | Raise to Verbose to capture solver convergence detail in the log file |

## Solver Comparison

| Solver | Typical memory per 1M DOF | Parallel scaling | Best for | When to pick |
|---|---|---|---|---|
| Direct (PARDISO) | 2 to 8 GB | Up to 8 to 16 threads | Small to medium 3D models, stiff coupled systems | Model fits in RAM and robustness matters more than scale |
| Iterative (GMRES + AMG) | 0.3 to 1 GB | 32+ threads | Large 3D models, single-physics diffusion problems | Memory is the binding constraint and the operator is well conditioned |
| Segregated | Depends on groups | Scales with group count | Weakly coupled multiphysics | One fully coupled solve fails to converge |
| Fully coupled | 2 to 10 GB | 16 to 32 threads | Strongly coupled physics, small meshes | Coupling is stiff and segregated stalls |

## Keyboard Shortcuts

| Action | Shortcut |
|---|---|
| Open Model Builder tree search | `Ctrl+F` |
| Save model | `Ctrl+S` |
| Compute current study | `F8` |
| Update solution (reuse assembly) | `Ctrl+F8` |
| Clear solution data | `F9` |
| Delete selected node | `Delete` |
| Duplicate selected node | `Ctrl+D` |
| Toggle axis and grid in graphics | `Ctrl+Shift+G` |
| Zoom to fit graphics window | `Ctrl+Shift+Z` |
| Copy current plot to clipboard | `Ctrl+C` in Graphics window |

## Command-Line Options

| Flag | Argument | Purpose |
|---|---|---|
| `-batch` | `study_tag` | Runs the named study headless and exits without opening the desktop |
| `-inputfile` | `model.mph` | Loads the specified model file before any batch action |
| `-outputfile` | `result.mph` | Writes the solved model to the given path after batch completion |
| `-pname` | `param=value` | Overrides a global parameter for a parametric batch run |
| `-plist` | `value1,value2` | Supplies a comma-separated list for the named `-pname` parameter |
| `-nn` | `node_count` | Requests a number of compute nodes from the cluster profile |
| `-ckl` | `license_feature` | Checks out a specific license feature before solving |
| `-silent` | none | Suppresses the splash window and dialogs in unattended runs |

```bash
comsolbatch -inputfile "C:\models\heat_sink.mph" -batch std1 -pname fin_thickness=2mm -outputfile "C:\models\out.mph" -silent
```

| Exit Code | Meaning |
|---|---|
| 0 | Batch completed and output file written |
| 1 | Solve failed to converge or license checkout failed |
| 2 | Input file missing or unreadable |
| 3 | Invalid parameter or flag argument |

## Troubleshooting

### Graphics window renders black or blank

Plots appear empty after a solve, often on laptops with hybrid graphics or remote sessions.

1. Open Preferences under Options and set Graphics rendering to Software OpenGL.
2. Force the discrete GPU for `comsol.exe` in Windows Graphics settings.
3. Update the GPU driver and confirm OpenGL 3.3 or later is reported.
4. Restart the desktop and re-open the plot group.

### Batch solve fails with license checkout error

`comsolbatch` exits with code 1 and logs a license feature checkout failure.

1. Verify the license server responds on port 1718 using `lmutil lmstat`.
2. Confirm the licensed feature name in the error matches your entitlement.
3. Check that no other machine has exhausted a floating seat.
4. Re-run with `-ckl` pointing at an available feature to isolate the failure.

### Out of memory during assembly

Windows raises a memory error or the solver stalls at the assembly stage on large meshes.

1. Switch the linear solver to iterative with AMG preconditioning.
2. Reduce mesh element order or apply symmetry and periodic boundary conditions.
3. Increase the Windows page file on the drive holding `%TEMP%`.
4. Move the solver workspace to an SSD with at least twice the model RAM footprint free.

### Recovery file does not resume

A restart after a crash offers no recovery prompt for an interrupted transient study.

1. Confirm files exist under `%USERPROFILE%\.comsol\recovery` with a recent timestamp.
2. Lower the recovery interval in Preferences and re-run to confirm the setting persists.
3. Check free disk space on the drive holding the recovery directory.
4. Reopen the model, then use Study > Recover from the recovery directory manually.

## FAQ

### Can one model file carry results from multiple solver configurations?

Yes. The Model Builder stores each study with its own solver sequence, so a model can hold a direct-solver study and an iterative-solver study side by side. Results attach to the study that produced them, and you switch between them in the Results node without duplicating geometry or physics.

### How do I move a licensed session to a machine without internet access?

Use the offline license dialog during install and supply the signed license file issued by the vendor, or replicate the `%PROGRAMDATA%\COMSOL` license configuration from a networked machine. Disable the update check in Preferences so the desktop does not attempt an outbound connection at startup.

### Does the Application Builder app run without the full desktop?

Yes. Compiled apps run in the COMSOL Client, which loads the model and exposes only the inputs and results the builder defined. The Client still performs license checkout for the underlying physics modules, so entitlements must be present on the machine running the app.

## Notes

COMSOL Multiphysics and COMSOL Desktop are trademarks of COMSOL AB. Windows 10 and Windows 11 are trademarks of Microsoft Corporation. This page documents installation, configuration, and command behavior for the Windows build and is not affiliated with or endorsed by COMSOL AB.